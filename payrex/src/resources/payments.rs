//! Payments API
//!
//! Payments represent successful payment transactions.

use crate::{
    Result,
    http::HttpClient,
    resources::customers::Customer,
    types::{Currency, Metadata, PaymentId, PaymentIntentId, PaymentMethod, Timestamp},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Payments API
#[derive(Clone)]
pub struct Payments {
    http: Arc<HttpClient>,
}

impl Payments {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Retrieve a Payment resource by ID.
    ///
    /// Endpoint: `GET /payments/:id`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/payments/retrieve)
    pub async fn retrieve(&self, id: &PaymentId) -> Result<Payment> {
        self.http.get(&format!("/payments/{}", id.as_str())).await
    }

    /// Update a Payment resource by ID.
    ///
    /// Endpoint: `PUT /payments/:id`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/payments/update)
    pub async fn update(&self, id: &PaymentId, params: UpdatePayment) -> Result<Payment> {
        self.http
            .patch(&format!("/payments/{}", id.as_str()), &params)
            .await
    }
}

/// The Payment resource represents an individual attempt to move money to your PayRex merchant
/// account balance.
///
/// When your customer successfully completed a transaction, a Payment resource represents the
/// actual payment of your customer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Payment {
    /// Unique identifier for the resource. The prefix is `pay_`.
    pub id: PaymentId,

    /// The amount of the payment to be transferred to your PayRex merchant account. This is a
    /// positive integer that your customer paid in the smallest currency unit, cents. If the
    /// customer paid ₱ 120.50, the amount of the Payment should be 12050.
    ///
    /// The minimum amount is ₱ 20 (2000 in cents) and the maximum amount is ₱ 59,999,999.99
    /// (5999999999 in cents).
    pub amount: u64,

    /// If the payment is either partially or fully refunded, the `amount_refunded` represents the
    /// successful refunded attempts. This is a positive integer that you can refund from the
    /// available amount of the Payment resource.
    pub amount_refunded: u64,

    #[allow(missing_docs)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<Billing>,

    /// A three-letter ISO currency code in uppercase. As of the moment, we only support PHP.
    pub currency: Currency,

    /// An arbitrary string attached to the Payment. Useful reference when viewing Payment from
    /// PayRex Dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The fee that PayRex will deduct from the amount of the Payment. This is a positive integer
    /// in the smallest currency unit, cents. If the fee is ₱ 120.50, the fee of the Payment should
    /// be 12050.
    pub fee: i64,

    /// The value is `true` if the resource's mode is live or the value is `false` if the resource is
    /// in test mode.
    pub livemode: bool,

    /// A set of key-value pairs attached to the Payment. This is useful for storing additional
    /// information about the Payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The `net_amount` of the payment is the final computed amount that will be transferred to the
    /// bank account of the merchant. This is a positive integer in the smallest currency unit,
    /// cents. If the `net_amount` is ₱ 120.50, the `net_amount` of the Payment should be 12050.
    pub net_amount: i64,

    /// The ID of the [`PaymentIntent`] resource that generated the Payment resource.
    pub payment_intent_id: PaymentIntentId,

    /// The status of the Payment. Possible values are `paid`, or `failed`.
    pub status: PaymentStatus,

    /// The Customer resource related to the Payment resource. If the payment does not have a
    /// customer resource, the value is null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,

    /// Holds the details of the payment method of the Payment.
    pub payment_method: PaymentMethodTypes,

    /// Defines if the payment is already refunded or not. A partially or fully refunded Payment
    /// has corresponding Refund resources. The value is `true` if the payment is either partially or
    /// fully refunded while the value is `false` if the payment has no refunds.
    pub refunded: bool,

    /// The time the resource was created and measured in seconds since the Unix epoch.
    pub created_at: Timestamp,

    /// The time the resource was updated and measured in seconds since the Unix epoch.
    pub updated_at: Timestamp,
}

/// Contains the billing information of the customer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Billing {
    /// The billing name of the customer.
    pub name: String,

    /// The billing e-mail address of the customer.
    pub email: String,

    /// The billing phone of the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// The billing address of the customer
    pub address: Address,
}

/// Contains the billing address of the customer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Address {
    /// The billing Address Line1 of the customer
    pub line1: Option<String>,

    /// The billing address line2 of the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// The billing address city of the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// The billing address state of the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// The billing address postal code of the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// The billing address country of the customer in two-letter ISO format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

/// Type of payment method with additional metadata for cards.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaymentMethodTypes {
    /// Defines the payment method of the Payment.
    #[serde(rename = "type")]
    pub method_type: PaymentMethod,

    /// Additional metadata included if the `type` is card.
    pub card: Option<PaymentMethodTypesCard>,
}

/// This is only visible if the `payment_method.type` is card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaymentMethodTypesCard {
    /// The first 6 digits of the card used to complete a payment
    pub first6: String,

    /// The last 4 digits of the card used to complete a payment
    pub last4: String,

    /// The brand of the card used to complete a payment
    pub brand: String,
}

/// Represents the status of a payment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    /// The payment transaction is successfully paid
    Paid,

    /// The payment transaction failed
    Failed,
}

/// Query parameters when updating a payment.
///
/// [Reference](https://docs.payrexhq.com/docs/api/payments/update#parameters)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdatePayment {
    /// An arbitrary string attached to the Payment. Useful reference when viewing Payment from
    /// PayRex Dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// A set of key-value pairs you can attach to the resource. This can be useful for storing
    /// additional information about the payment in a hash format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl UpdatePayment {
    /// Creates a new [`UpdatePayment`] instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the description in the query params for updating a payment.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the metadata in the query params for updating a payment.
    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_payment_builder() {
        let mut metadata = Metadata::new();
        metadata.insert("order_id".to_string(), "12345".to_string());

        let params = UpdatePayment::new()
            .description("Test payment")
            .metadata(metadata.clone());

        assert_eq!(params.description, Some("Test payment".to_string()));
        assert_eq!(params.metadata, Some(metadata));
    }

    #[test]
    fn test_payment_status_serialization() {
        let status = PaymentStatus::Paid;
        let serialized = serde_json::to_string(&status).unwrap();
        assert_eq!(serialized, "\"paid\"");

        let status = PaymentStatus::Failed;
        let serialized = serde_json::to_string(&status).unwrap();
        assert_eq!(serialized, "\"failed\"");
    }

    #[test]
    fn test_address_serialization() {
        let address = Address {
            line1: Some("BGC".to_string()),
            line2: Some("Apt 4B".to_string()),
            city: Some("Taguig".to_string()),
            state: Some("NCR".to_string()),
            postal_code: Some("1635".to_string()),
            country: Some("PH".to_string()),
        };

        let serialized = serde_json::to_string(&address).unwrap();
        let expected = r#"{"line1":"BGC","line2":"Apt 4B","city":"Taguig","state":"NCR","postal_code":"1635","country":"PH"}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_billing_serialization() {
        let address = Address {
            line1: Some("BGC".to_string()),
            line2: Some("Apt 4B".to_string()),
            city: Some("Taguig".to_string()),
            state: Some("NCR".to_string()),
            postal_code: Some("1635".to_string()),
            country: Some("PH".to_string()),
        };

        let billing = Billing {
            name: "John Doe".to_string(),
            email: "johndoe@gmail.com".to_string(),
            phone: Some("1234567890".to_string()),
            address,
        };

        let serialized = serde_json::to_string(&billing).unwrap();
        let expected = r#"{"name":"John Doe","email":"johndoe@gmail.com","phone":"1234567890","address":{"line1":"BGC","line2":"Apt 4B","city":"Taguig","state":"NCR","postal_code":"1635","country":"PH"}}"#;

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_payment_method_types_serialization() {
        let payment_method = PaymentMethodTypes {
            method_type: PaymentMethod::Card,
            card: Some(PaymentMethodTypesCard {
                first6: "511263".to_string(),
                last4: "2710".to_string(),
                brand: "MasterCard".to_string(),
            }),
        };

        let serialized = serde_json::to_string(&payment_method).unwrap();
        let expected =
            r#"{"type":"card","card":{"first6":"511263","last4":"2710","brand":"MasterCard"}}"#;

        assert_eq!(serialized, expected);
    }
}
