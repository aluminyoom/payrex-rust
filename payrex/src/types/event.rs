use std::fmt::Display;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

use crate::types::{EventId, Timestamp};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    pub id: EventId,
    pub data: Value,
    #[serde(rename = "type")]
    pub event_type: EventType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_webhooks: Option<u64>,
    pub livemode: bool,
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub previous_attributes: Option<Value>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventType {
    BillingStatement(BillingStatementEvent),
    BillingStatementLineItem(BillingStatementLineItemEvent),
    CheckoutSession(CheckoutSessionEvent),
    PaymentIntent(PaymentIntentEvent),
    Payout(PayoutEvent),
    Refund(RefundEvent),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BillingStatementEvent {
    Created,
    Updated,
    Deleted,
    Finalized,
    Sent,
    MarkedUncollectible,
    Voided,
    Paid,
    WillBeDue,
    Overdue,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BillingStatementLineItemEvent {
    Created,
    Updated,
    Deleted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionEvent {
    Expired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentEvent {
    AwaitingCapture,
    Succeeded,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayoutEvent {
    Deposited,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefundEvent {
    Created,
    Updated,
}

impl Serialize for EventType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            EventType::BillingStatement(e) => format!("billing_statement.{e:?}"),
            EventType::BillingStatementLineItem(e) => {
                format!("billing_statement_line_item.{e:?}")
            }
            EventType::CheckoutSession(e) => format!("checkout_session.{e:?}"),
            EventType::PaymentIntent(e) => format!("payment_intent.{e:?}"),
            EventType::Payout(e) => format!("payout.{e:?}"),
            EventType::Refund(e) => format!("refund.{e:?}"),
        };
        serializer.serialize_str(&s.to_lowercase())
    }
}

impl<'de> Deserialize<'de> for EventType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 2 {
            return Err(serde::de::Error::custom("invalid event format"));
        }

        let (prefix, event) = (parts[0], parts[1]);
        Ok(match prefix {
            "billing_statement" => EventType::BillingStatement(
                serde_plain::from_str(event).map_err(serde::de::Error::custom)?,
            ),
            "billing_statement_line_item" => EventType::BillingStatementLineItem(
                serde_plain::from_str(event).map_err(serde::de::Error::custom)?,
            ),
            "checkout_session" => EventType::CheckoutSession(
                serde_plain::from_str(event).map_err(serde::de::Error::custom)?,
            ),
            "payment_intent" => EventType::PaymentIntent(
                serde_plain::from_str(event).map_err(serde::de::Error::custom)?,
            ),
            "payout" => {
                EventType::Payout(serde_plain::from_str(event).map_err(serde::de::Error::custom)?)
            }
            "refund" => {
                EventType::Refund(serde_plain::from_str(event).map_err(serde::de::Error::custom)?)
            }
            _ => return Err(serde::de::Error::custom("unknown event type")),
        })
    }
}

impl EventType {
    #[must_use]
    pub fn as_str(&self) -> String {
        serde_plain::to_string(&self).unwrap()
    }
}

impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_event_type_serialization_and_as_str() {
        // Simple event type serialization and Display
        let et = EventType::BillingStatement(BillingStatementEvent::Created);
        assert_eq!(et.as_str(), "billing_statement.created");
        assert_eq!(
            serde_json::to_string(&et).unwrap(),
            "\"billing_statement.created\""
        );
        assert_eq!(format!("{et}"), "billing_statement.created");

        // Another variant
        let et2 = EventType::Refund(RefundEvent::Updated);
        assert_eq!(et2.as_str(), "refund.updated");
        assert_eq!(serde_json::to_string(&et2).unwrap(), "\"refund.updated\"");
    }

    #[test]
    fn test_event_serialization() {
        let id = EventId::new("evt_123");
        let data = json!({"key": "value"});
        let event = Event {
            id: id.clone(),
            data: data.clone(),
            event_type: EventType::CheckoutSession(CheckoutSessionEvent::Expired),
            pending_webhooks: Some(3),
            livemode: false,
            created_at: Timestamp::from_unix(1_600_000_000),
            updated_at: Timestamp::from_unix(1_600_000_500),
        };

        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(json["id"], id.as_str());
        assert_eq!(json["data"], data);
        assert_eq!(json["type"], "checkout_session.expired");
        assert_eq!(json["pending_webhooks"], 3);
        assert_eq!(json["livemode"], false);
        assert_eq!(json["created_at"], 1_600_000_000);
        assert_eq!(json["updated_at"], 1_600_000_500);
    }
}
