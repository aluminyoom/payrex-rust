//! Payouts API
//!
//! Payouts represent transfers of funds to your bank account.

use crate::{
    Result,
    http::HttpClient,
    types::{List, ListParams, PayoutId, PayoutTransactionId, Timestamp},
};
use payrex_derive::payrex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Payouts API
#[derive(Clone)]
pub struct Payouts {
    http: Arc<HttpClient>,
}

impl Payouts {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// List payout transaction resources.
    ///
    /// Endpoint: `GET /payouts/:id/transactions`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/payout_transactions/list)
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

/// The Payout resource is created when you are scheduled to receive money from PayRex. Payouts are
/// made depending on the payout schedule for your PayRex merchant account. A Payout resource
/// represents a net amount of money settled to your nominated bank account.
#[payrex(amount, livemode, timestamp)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Payout {
    /// Unique identifier for the resource. The prefix is `po_`.
    pub id: PayoutId,

    /// The destination attribute holds the information of the bank account that you nominated for
    /// your PayRex merchant account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<PayoutDestination>,

    /// The net_amount of the Payout is the final computed amount that will be transferred to the
    /// bank account associated with the PayRex merchant account. This is a positive integer in the
    /// smallest currency unit, cents. If the net_amount is ₱ 120.50, the net_amount of the Payout
    /// should be 12050.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<u64>,

    /// The status of the Payout. Possible values are `pending`, `in_transit`, `failed`, or
    /// `successful`.
    pub status: PayoutStatus,
}

/// The status of a Payout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayoutStatus {
    /// The payout is currently pending.
    Pending,

    /// The payout is in transit.
    InTransit,

    /// The payout failed.
    Failed,

    /// The payout was cancelled.
    Cancelled,
}

/// The payout destination holds the information of the bank account that you nominated for
/// your PayRex merchant account.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PayoutDestination {
    /// The account name of the bank account
    pub account_name: String,

    /// The account number of the bank account.
    pub account_number: String,

    /// The name of the bank.
    pub bank_name: String,
}

/// The transaction type of a Payout Transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayoutTransactionType {
    /// The transaction type is a payment.
    Payment,

    /// The transaction type is a refund.
    Refund,

    /// The transaction type is an adjustment.
    Adjustment,
}

/// The Payment Transaction resource represents every line item of a Payout. Every Payout
/// Transaction belongs to a Payout resource.
#[payrex(amount, timestamp)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PayoutTransaction {
    /// Unique identifier for the resource. The prefix is `po_txn_`.
    pub id: PayoutTransactionId,

    /// The net amount of the Payout Transaction.
    ///
    /// If the transaction_type is payment, the value is the gross amount less the deductibles such
    /// as fees and tax. If the transaction_type is refund, the value is the successfully refunded
    /// amount. If the transaction_type is adjustment, the value is the debit or credit adjustment
    /// from the Payout.
    ///
    /// This is a positive or negative integer in the smallest currency unit, cents. Positive
    /// integer is credit to your final payout while negative integer is debit to your final
    /// payout.
    ///
    /// If the net amount is ₱ 120.50, the net amount of the Payout Transaction should be 12050.
    pub net_amount: u64,

    /// The `transaction_id` is the unique identifier of the Payout Transaction's transaction type.
    ///
    /// If the `transaction_type` is payment, it is the ID of the Payment resource.
    ///
    /// If the `transaction_type` is refund, it is the ID of the Refund resource.
    ///
    /// If the `transaction_type` is adjustment, it is the ID of the Adjustment resource.
    // TODO: identify the type of resource id based on `transaction_type`
    pub transaction_id: PayoutTransactionId,

    /// The transaction type of the Payout Transaction. The possible values are `payment`, `refund`,
    /// and `adjustment`.
    pub transaction_type: PayoutTransactionType,
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
            updated_at: Timestamp::from_unix(1_610_001_000),
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
            id: PayoutTransactionId::new("po_txn_abc"),
            amount: 500,
            net_amount: 490,
            transaction_id: PayoutTransactionId::new("po_txn_xyz"),
            transaction_type: PayoutTransactionType::Refund,
            created_at: Timestamp::from_unix(1_610_002_000),
            updated_at: Timestamp::from_unix(1_610_002_000),
        };
        let json = serde_json::to_value(&tx).unwrap();
        assert_eq!(json["id"], "po_txn_abc");
        assert_eq!(json["amount"], 500);
        assert_eq!(json["net_amount"], 490);
        assert_eq!(json["transaction_id"], "po_txn_xyz");
        assert_eq!(json["transaction_type"], "refund");
        assert_eq!(json["created_at"], 1_610_002_000);
        assert_eq!(json["updated_at"], 1_610_002_000);
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
}
