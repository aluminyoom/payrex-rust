//! Refunds API
//!
//! Refunds allow you to return money to a customer.

use crate::{
    Result,
    http::HttpClient,
    types::{Currency, Metadata, PaymentId, RefundId, Timestamp},
};
use payrex_derive::payrex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Refunds API
#[derive(Clone)]
pub struct Refunds {
    http: Arc<HttpClient>,
}

impl Refunds {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Creates a Refund resource.
    ///
    /// Endpoint: `POST /refunds`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/refunds/create)
    pub async fn create(&self, params: CreateRefund) -> Result<Refund> {
        self.http.post("/refunds", &params).await
    }

    /// Updates a Refund resource.
    ///
    /// Endpoint: `PUT /refunds/:id`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/refunds/update)
    pub async fn update(&self, id: &RefundId, params: UpdateRefund) -> Result<Refund> {
        self.http
            .put(&format!("/refunds/{}", id.as_str()), &params)
            .await
    }
}

/// A Refund resource represents a refunded amount of a paid payment.
#[payrex(
    timestamp,
    metadata,
    amount,
    currency,
    livemode,
    description = "refund"
)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Refund {
    /// Unique identifier for the resource. The prefix is `re_`.
    pub id: RefundId,

    /// The latest status of the Refund. Possible values are `succeeded`, `failed`, or `pending`.
    pub status: RefundStatus,

    /// The reason of the Refund. Possible values are `fraudulent`, `requested_by_customer`,
    /// `product_out_of_stock`, `service_not_provided`, `product_was_damaged`, `service_misaligned`,
    /// `wrong_product_received`, or `others`.
    ///
    /// You can use the `remarks` attribute to add remarks if the
    /// value of the reason is `others`.
    pub reason: RefundReason,

    /// Remarks about the Refund resource. This is useful when viewing a refund via PayRex
    /// Dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,

    /// The ID of the payment to be refunded.
    pub payment_id: PaymentId,
}

/// The latest status of a Refund.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefundStatus {
    /// Refund status when a refund is pending.
    Pending,

    /// Refund status when a refund succeeded.
    Succeeded,

    /// Refund status when a refund failed.
    Failed,
}

/// The reason of a Refund.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefundReason {
    /// The reason for a refund is from a fraudulent payment.
    Fraudulent,

    /// The reason for a refund is when it is requested by a customer.
    RequestedByCustomer,

    /// The reason for a refund is when the product is out of stock.
    ProductOutOfStock,

    /// The reason for a refund is when the product was damaged.
    ProductWasDamaged,

    /// The reason for a refund is when the service was not provided to the customer.
    ServiceNotProvided,

    /// The reason for a refund is when the service is misaligned.
    ServiceMisaligned,

    /// The reason for a refund is when the product received by a customer is a wrong.
    WrongProductReceived,

    /// The reason for a refund is indicated in the remarks.
    Others,
}

/// Query parameters when creating a refund.
///
/// [Reference](https://docs.payrexhq.com/docs/api/refunds/create#parameters)
#[payrex(amount, currency, metadata, description = "refund")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRefund {
    /// The ID of the payment to be refunded.
    pub payment_id: PaymentId,

    /// The reason of the Refund. Possible values are `fraudulent`, `requested_by_customer`,
    /// `product_out_of_stock`, `service_not_provided`, `product_was_damaged`, `service_misaligned`,
    /// `wrong_product_received`, or `others`.
    ///
    /// You can use the `remarks` attribute to add remarks if the
    /// value of the reason is `others`.
    pub reason: RefundReason,

    /// Remarks about the Refund resource. This is useful when viewing a refund via PayRex
    /// Dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
}

/// Query parameters when updating a refund.
///
/// [Reference](https://docs.payrexhq.com/docs/api/refunds/update#parameters)
#[payrex(metadata)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateRefund {}

impl CreateRefund {
    /// Creates a new [`CreateRefund`] instance.
    #[must_use]
    pub fn new(
        payment_id: PaymentId,
        amount: u64,
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

    /// Sets the metadata in the query params when creating a refund.
    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Sets the remarks when refund status is set to `others` when creating a refund.
    pub fn remarks(mut self, remarks: impl Into<String>) -> Self {
        self.remarks = Some(remarks.into());
        self
    }

    /// Sets the description in the query params when creating a refund.
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
