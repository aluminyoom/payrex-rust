//! Payouts API
//!
//! Payouts represent transfers of funds to your bank account.

use crate::{
    Result,
    http::HttpClient,
    types::{List, ListParams, PayoutId, PayoutTransactionId, Timestamp},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct Payouts {
    http: Arc<HttpClient>,
}

impl Payouts {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub async fn list_transactions(
        &self,
        id: &PayoutId,
        params: Option<ListParams>,
    ) -> Result<List<PayoutTransaction>> {
        self.http
            .get_with_params(&format!("/payouts/{}/transactions", id.as_str()), &params)
            .await
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Payout {
    pub id: PayoutId,
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<PayoutDestination>,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<i64>,
    pub status: PayoutStatus,
    pub created_at: Timestamp,
    pub updated_at: Option<Timestamp>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayoutStatus {
    Pending,
    InTransit,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PayoutDestination {
    pub account_name: String,
    pub account_number: String,
    pub bank_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayoutTransactionType {
    Payment,
    Refund,
    Adjustment,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PayoutTransaction {
    pub id: PayoutTransactionId,
    pub amount: i32,
    pub net_amount: i32,
    // TODO: identify the type of resource id based on `transaction_type`
    pub transaction_id: PayoutTransactionId,
    pub transaction_type: PayoutTransactionType,
    pub created_at: Timestamp,
    pub updated_at: Option<Timestamp>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{PayoutId, PayoutTransactionId, Timestamp};
    use serde_json;

    #[test]
    fn test_payout_status_serialization() {
        let status = PayoutStatus::Pending;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"pending\"");

        let status = PayoutStatus::InTransit;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"in_transit\"");

        let status = PayoutStatus::Failed;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"failed\"");

        let status = PayoutStatus::Cancelled;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"cancelled\"");
    }

    #[test]
    fn test_payout_transaction_type_serialization() {
        let kind = PayoutTransactionType::Payment;
        assert_eq!(serde_json::to_string(&kind).unwrap(), "\"payment\"");
        let kind = PayoutTransactionType::Refund;
        assert_eq!(serde_json::to_string(&kind).unwrap(), "\"refund\"");
        let kind = PayoutTransactionType::Adjustment;
        assert_eq!(serde_json::to_string(&kind).unwrap(), "\"adjustment\"");
    }

    #[test]
    fn test_payout_serialization() {
        let dest = PayoutDestination {
            account_name: "John Doe".to_string(),
            account_number: "123456".to_string(),
            bank_name: "Test Bank".to_string(),
        };
        let payout = Payout {
            id: PayoutId::new("po_123"),
            amount: 5000,
            destination: Some(dest.clone()),
            livemode: true,
            net_amount: Some(4900),
            status: PayoutStatus::Pending,
            created_at: Timestamp::from_unix(1_610_000_000),
            updated_at: Some(Timestamp::from_unix(1_610_001_000)),
        };
        let json = serde_json::to_value(&payout).unwrap();
        assert_eq!(json["id"], "po_123");
        assert_eq!(json["amount"], 5000);
        assert_eq!(json["destination"]["account_name"], "John Doe");
        assert_eq!(json["destination"]["account_number"], "123456");
        assert_eq!(json["destination"]["bank_name"], "Test Bank");
        assert_eq!(json["livemode"], true);
        assert_eq!(json["net_amount"], 4900);
        assert_eq!(json["status"], "pending");
        assert_eq!(json["created_at"], 1_610_000_000);
        assert_eq!(json["updated_at"], 1_610_001_000);
    }

    #[test]
    fn test_payout_transaction_serialization() {
        let tx = PayoutTransaction {
            id: PayoutTransactionId::new("pot_abc"),
            amount: 500,
            net_amount: 490,
            transaction_id: PayoutTransactionId::new("pot_xyz"),
            transaction_type: PayoutTransactionType::Refund,
            created_at: Timestamp::from_unix(1_610_002_000),
            updated_at: None,
        };
        let json = serde_json::to_value(&tx).unwrap();
        assert_eq!(json["id"], "pot_abc");
        assert_eq!(json["amount"], 500);
        assert_eq!(json["net_amount"], 490);
        assert_eq!(json["transaction_id"], "pot_xyz");
        assert_eq!(json["transaction_type"], "refund");
        assert_eq!(json["created_at"], 1_610_002_000);
        assert!(json.get("updated_at").unwrap().is_null());
    }
}
#[test]
fn test_payout_destination_serialization() {
    let dest = PayoutDestination {
        account_name: "Jane Roe".to_string(),
        account_number: "654321".to_string(),
        bank_name: "Example Bank".to_string(),
    };
    let serialized = serde_json::to_string(&dest).unwrap();
    let expected =
        r#"{"account_name":"Jane Roe","account_number":"654321","bank_name":"Example Bank"}"#;
    assert_eq!(serialized, expected);
}
