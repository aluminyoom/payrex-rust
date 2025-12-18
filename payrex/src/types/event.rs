//! Event types in PayRex to be used in Webhooks endpoints.

use std::fmt::Display;

use payrex_derive::payrex_attr;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

use crate::types::{EventId, Timestamp};

/// An Event resource represents updates in your PayRex account triggered either by API calls or
/// your actions from the Dashboard. When an event occurs, for example, a successfully paid payment
/// is received, PayRex creates a new Event resource with type `payment_intent.succeeded`.
///
/// Events occur when the state or attribute values of different API resources change. An Event
/// resource's data attribute contains the resource's state or snapshot at the time of the change.
/// For example, a payment_intent.succeeded event contains a Payment Intent, and a payment.refunded
/// event contains a Payment resource.
#[payrex_attr(livemode, timestamp)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    /// Unique identifier for the resource. The prefix is `evt_`.
    pub id: EventId,

    /// Contains the resource associated with the event, and the previous values if the event is a
    /// resource update. For example, if the event type is `payment_intent.succeeded`, this will
    /// contain a `PaymentIntent` resource.
    pub data: Value,

    /// The type of the event.
    #[serde(rename = "type")]
    pub event_type: EventType,

    /// The number of webhooks that haven't been successfully delivered for a specific Event
    /// resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_webhooks: Option<u64>,
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub previous_attributes: Option<Value>,
}

/// The event types follow a pattern: `<resource>.<event>`. We aim to be consistent, making things
/// easier and more organized.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventType {
    /// Event types about Billing Statement.
    BillingStatement(BillingStatementEvent),

    /// Event types about Billing Statement Line Item.
    BillingStatementLineItem(BillingStatementLineItemEvent),

    /// Event types about Checkout Session.
    CheckoutSession(CheckoutSessionEvent),

    /// Event types about Payment Intent.
    PaymentIntent(PaymentIntentEvent),

    /// Event types about Payout.
    Payout(PayoutEvent),

    /// Event types about Refund.
    Refund(RefundEvent),
}

/// Event types about Billing Statement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BillingStatementEvent {
    /// Occurs when a Billing Statement has been created.
    ///
    /// Event type as string representation: `billing_statement.created`
    Created,

    /// Occurs when a Billing Statement has been updated. For example, if the due date has been
    /// updated.
    ///
    /// Event type as string representation: `billing_statement.updated`
    Updated,

    /// Occurs when a Billing Statement has been deleted.
    ///
    /// Event type as string representation: `billing_statement.deleted`
    Deleted,

    /// Occurs when a Billing Statement has been finalized.
    ///
    /// Event type as string representation: `billing_statement.finalized`
    Finalized,

    /// Occurs when a Billing Statement has been sent to the customer.
    ///
    /// Event type as string representation: `billing_statement.sent`
    Sent,

    /// Occurs when a Billing Statement has been marked as `uncollectible`.
    ///
    /// Event type as string representation: `billing_statement.marked_uncollectible`
    MarkedUncollectible,

    /// Occurs when a Billing Statement has been marked as `voided`.
    ///
    /// Event type as string representation: `billing_statement.voided`
    Voided,

    /// Occurs when a Billing Statement has been paid by the customer.
    ///
    /// Event type as string representation: `billing_statement.paid`
    Paid,

    /// Occurs 5 number of days before a billing statement becomes due.
    ///
    /// Event type as string representation: `billing_statement.will_be_due`
    WillBeDue,

    /// Occurs 5 number of days after a billing statement becomes due.
    ///
    /// Event type as string representation: `billing_statement.overdue`
    Overdue,
}

/// Event types about Billing Statement Line Item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BillingStatementLineItemEvent {
    /// Occurs when a Billing Statement Line Item is created.
    ///
    /// Event type as string representation: `billing_statement_line_item.created`
    Created,

    /// Occurs when a Billing Statement Line Item is updated.
    ///
    /// Event type as string representation: `billing_statement_line_item.updated`
    Updated,

    /// Occurs when a Billing Statement Line Item is deleted.
    ///
    /// Event type as string representation: `billing_statement_line_item.deleted`
    Deleted,
}

/// Event types about Checkout Session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionEvent {
    /// Occurs when a Checkout Session expires.
    ///
    /// Event type as string representation: `checkout_session.expired`
    Expired,
}

/// Event types about Payment Intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentEvent {
    /// Occurs when a Payment Intent can be captured. This event is applicable for [hold then
    /// capture](https://docs.payrexhq.com/docs/guide/developer_handbook/payments/payment_methods/card/hold_then_capture)
    /// feature for card payments.
    ///
    /// Event type as string representation: `payment_intent.awaiting_capture`
    AwaitingCapture,

    /// Occurs when the Payment Intent has successfully completed a payment. Another notable change
    /// is the Payment Intent status transitions to `succeeded`.
    ///
    /// Event type as string representation: `payment_intent.succeeded`
    Succeeded,
}

/// Event types about Payout.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayoutEvent {
    /// Occurs when a Payout transfer to your bank account is successful. Another notable change is
    /// the Payout status transitions to `successful`.
    ///
    /// Event type as string representation: `payout.deposited`
    Deposited,
}

/// Event types about Refund.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefundEvent {
    /// Occurs when a Payment is refunded, either partial or full amount. To determine the final
    /// status of a Refund resource, best to use refund.updated event.
    ///
    /// Event type as string representation: `refund.created`
    Created,

    /// Occurs when a Refund is updated, either partial or full amount. This event holds the final
    /// status of a refund.
    ///
    /// Event type as string representation: `refund.updated`
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
    /// Returns the String representation of an event type.
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
