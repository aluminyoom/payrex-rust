//! Refunds API
//!
//! Refunds allow you to return money to a customer.

use crate::{
    Result,
    http::HttpClient,
    types::{Currency, Metadata, PaymentId, RefundId, Timestamp},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct Refunds {
    http: Arc<HttpClient>,
}

impl Refunds {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub async fn create(&self, params: CreateRefund) -> Result<Refund> {
        self.http.post("/refunds", &params).await
    }

    pub async fn update(&self, id: &RefundId, params: UpdateRefund) -> Result<Refund> {
        self.http
            .put(&format!("/refunds/{}", id.as_str()), &params)
            .await
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Refund {
    pub id: RefundId,
    pub amount: i64,
    pub currency: Currency,
    pub livemode: bool,
    pub status: RefundStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub reason: RefundReason,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
    pub payment_id: PaymentId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefundStatus {
    Pending,
    Succeeded,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefundReason {
    Fraudulent,
    RequestedByCustomer,
    ProductOutOfStock,
    ProductWasDamaged,
    ServiceNotProvided,
    ServiceMisaligned,
    WrongProductReceived,
    Others,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRefund {
    pub payment_id: PaymentId,
    pub amount: i64,
    pub currency: Currency,
    pub reason: RefundReason,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateRefund {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl CreateRefund {
    #[must_use]
    pub fn new(
        payment_id: PaymentId,
        amount: i64,
        currency: Currency,
        reason: RefundReason,
    ) -> Self {
        Self {
            payment_id,
            amount,
            currency,
            reason,
            metadata: None,
            remarks: None,
            description: None,
        }
    }

    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn remarks(mut self, remarks: impl Into<String>) -> Self {
        self.remarks = Some(remarks.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Currency, Metadata, PaymentId, RefundId, Timestamp};
    use serde_json;

    #[test]
    fn test_refund_status_serialization() {
        assert_eq!(
            serde_json::to_string(&RefundStatus::Pending).unwrap(),
            "\"pending\""
        );
        assert_eq!(
            serde_json::to_string(&RefundStatus::Succeeded).unwrap(),
            "\"succeeded\""
        );
        assert_eq!(
            serde_json::to_string(&RefundStatus::Failed).unwrap(),
            "\"failed\""
        );
    }

    #[test]
    fn test_refund_reason_serialization() {
        assert_eq!(
            serde_json::to_string(&RefundReason::Fraudulent).unwrap(),
            "\"fraudulent\""
        );
        assert_eq!(
            serde_json::to_string(&RefundReason::RequestedByCustomer).unwrap(),
            "\"requested_by_customer\""
        );
        assert_eq!(
            serde_json::to_string(&RefundReason::WrongProductReceived).unwrap(),
            "\"wrong_product_received\""
        );
        assert_eq!(
            serde_json::to_string(&RefundReason::Others).unwrap(),
            "\"others\""
        );
    }

    #[test]
    fn test_refund_serialization() {
        let mut metadata = Metadata::new();
        metadata.insert("key", "value");

        let refund = Refund {
            id: RefundId::new("re_123"),
            amount: 1000,
            currency: Currency::PHP,
            livemode: false,
            status: RefundStatus::Succeeded,
            description: Some("desc".to_string()),
            reason: RefundReason::Fraudulent,
            remarks: Some("note".to_string()),
            payment_id: PaymentId::new("pay_456"),
            metadata: Some(metadata.clone()),
            created_at: Timestamp::from_unix(1_620_000_000),
            updated_at: Timestamp::from_unix(1_620_001_000),
        };

        let json = serde_json::to_value(&refund).unwrap();

        assert_eq!(json["id"], "re_123");
        assert_eq!(json["amount"], 1000);
        assert_eq!(json["currency"], "PHP");
        assert_eq!(json["livemode"], false);
        assert_eq!(json["status"], "succeeded");
        assert_eq!(json["description"], "desc");
        assert_eq!(json["reason"], "fraudulent");
        assert_eq!(json["remarks"], "note");
        assert_eq!(json["payment_id"], "pay_456");
        assert_eq!(json["metadata"]["key"], "value");
        assert_eq!(json["created_at"], 1_620_000_000);
        assert_eq!(json["updated_at"], 1_620_001_000);
    }

    #[test]
    fn test_create_refund_builder() {
        let mut metadata = Metadata::new();
        metadata.insert("order", "1");

        let params = CreateRefund::new(
            PaymentId::new("pay_abc"),
            123,
            Currency::PHP,
            RefundReason::WrongProductReceived,
        )
        .metadata(metadata.clone())
        .remarks("note")
        .description("desc");
        assert_eq!(params.payment_id.as_str(), "pay_abc");
        assert_eq!(params.amount, 123);
        assert_eq!(params.currency, Currency::PHP);
        assert_eq!(params.reason, RefundReason::WrongProductReceived);
        assert_eq!(params.metadata.unwrap().get("order"), Some("1"));
        assert_eq!(params.remarks, Some("note".to_string()));
        assert_eq!(params.description, Some("desc".to_string()));
    }

    #[test]
    fn test_update_refund_serialization() {
        let mut metadata = Metadata::new();
        metadata.insert("foo", "bar");
        let params = UpdateRefund {
            metadata: Some(metadata.clone()),
        };
        let serialized = serde_json::to_string(&params).unwrap();
        assert_eq!(serialized, r#"{"metadata":{"foo":"bar"}}"#);
    }
}
