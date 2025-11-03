//! Payment Intents API
//!
//! Payment Intents represent an intent to collect payment from a customer.
//! They track the lifecycle of a payment from creation through completion.

use crate::{
    Result,
    http::HttpClient,
    types::{
        CaptureMethod, Currency, Metadata, PaymentIntentId, PaymentMethod, PaymentMethodOptions,
        Timestamp,
    },
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
/// A [`PaymentIntent`] tracks the customer's payment lifecycle, keeping track of any failed payment
/// attempts and ensuring the customer is only charged once. Create one [`PaymentIntent`] whenever your
/// customer arrives at your checkout page. Retrieve the Payment Intent later to see the history of
/// payment attempts.
pub struct PaymentIntents {
    http: Arc<HttpClient>,
}

impl PaymentIntents {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Creates a [`PaymentIntent`] resource.
    ///
    /// Endpoint: `POST /payment_intents`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/payment_intents/create)
    pub async fn create(&self, params: CreatePaymentIntent) -> Result<PaymentIntent> {
        self.http.post("/payment_intents", &params).await
    }

    /// Retrieve a [`PaymentIntent`] resource by ID.
    ///
    /// Endpoint: `GET /payment_intents/:id`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/payment_intents/retrieve)
    pub async fn retrieve(&self, id: &PaymentIntentId) -> Result<PaymentIntent> {
        self.http
            .get(&format!("/payment_intents/{}", id.as_str()))
            .await
    }

    /// Cancels a [`PaymentIntent`] resource. A payment intent with a status of `canceled` means your
    /// customer cannot proceed with paying the particular payment intent.
    ///
    /// Endpoint: `POST /payment_intents/:id/cancel`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/payment_intents/cancel)
    pub async fn cancel(&self, id: &PaymentIntentId) -> Result<PaymentIntent> {
        self.http
            .post(&format!("/payment_intents/{}/cancel", id.as_str()), &())
            .await
    }

    /// Captures a [`PaymentIntent`] resource.
    ///
    /// Endpoint: `POST /payment_intents/:id/capture`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/payment_intents/capture)
    pub async fn capture(
        &self,
        id: &PaymentIntentId,
        params: CapturePaymentIntent,
    ) -> Result<PaymentIntent> {
        self.http
            .post(
                &format!("/payment_intents/{}/capture", id.as_str()),
                &params,
            )
            .await
    }
}

/// If this attribute is present, it tells you what actions you need to take so that your customer
/// can make a payment using the selected method.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NextAction {
    /// The type of the next action to perform, The possible value is `redirect`.
    #[serde(rename = "type")]
    pub action_type: String,

    /// The URL for authenticating a payment by redirecting your customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
}

/// The error code returned in case of a failed payment attempt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaymentError {
    /// The status code of the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// A message that provides more details about the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// If the error is parameter-specific, the parameter related to the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
}

/// A [`PaymentIntent`] tracks the customer's payment lifecycle, keeping track of any failed payment attempts and ensuring the customer is only charged once. Create one [`PaymentIntent`] whenever your customer arrives at your checkout page. Retrieve the Payment Intent later to see the history of payment attempts.
///
/// A [`PaymentIntent`] transitions through multiple statuses throughout its lifetime via Payrex.JS until it creates, at most, one successful payment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaymentIntent {
    /// Unique identifier for the resource. The prefix is `pi_`.
    pub id: PaymentIntentId,

    /// The amount to be collected by the [`PaymentIntent`]. This is a positive integer that your
    /// customer will pay in the smallest currency unit, cents. If the customer should pay ₱
    /// 120.50, the amount of the [`PaymentIntent`] should be 12050.
    ///
    /// The minimum amount is ₱ 20 (2000 in cents) and the maximum amount is ₱ 59,999,999.99
    /// (5999999999 in cents).
    pub amount: i64,

    /// The amount already collected by the [`PaymentIntent`]. This is a positive integer that your
    /// customer paid in the smallest currency unit, cents. If the customer paid ₱ 120.50, the
    /// `amount_received` of the [`PaymentIntent`] should be 12050.
    ///
    /// The minimum amount is ₱ 20 (2000 in cents) and the maximum amount is ₱ 59,999,999.99
    /// (5999999999 in cents).
    pub amount_received: i64,

    /// The amount that can be captured by the [`PaymentIntent`]. This is a positive integer that your
    /// customer authorized in the smallest currency unit, cents. If the customer authorized ₱
    /// 120.50, the `amount_capturable` of the [`PaymentIntent`] should be 12050.
    ///
    /// The minimum amount is ₱ 20 (2000 in cents) and the maximum amount is ₱ 59,999,999.99
    /// (5999999999 in cents).
    pub amount_capturable: i64,

    ///The client secret of this [`PaymentIntent`] used for client-side retrieval using a public API
    ///key. The client secret can be used to complete a payment from your client application.
    pub client_secret: String,

    /// A three-letter ISO currency code in uppercase. As of the moment, we only support PHP.
    pub currency: Currency,

    /// An arbitrary string attached to the [`PaymentIntent`]. Useful reference when viewing paid
    /// Payment from PayRex Dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The value is `true` if the resource's mode is live or the value is `false` if the resource mode is test.
    pub livemode: bool,

    /// A set of key-value pairs attached to the [`PaymentIntent`] and the resources created by the
    /// [`PaymentIntent`], e.g., Payment. This is useful for storing additional information about the
    /// [`PaymentIntent`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The `Payment` ID of the latest successful payment created by the [`PaymentIntent`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_payment: Option<String>,

    /// The error returned in case of a failed payment attempt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_payment_error: Option<PaymentError>,

    /// The latest `PaymentMethod` ID of attached to the [`PaymentIntent`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,

    /// The list of payment methods allowed to be processed by the [`PaymentIntent`].
    pub payment_methods: Vec<String>,

    /// A set of key-value pairs that can modify the behavior of the payment method attached to the
    /// payment intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<PaymentMethodOptions>,

    /// Text that appears on the customer's bank statement. This value overrides the merchant
    /// account's trade name. For information about requirements, including the 22-character limit,
    /// see the [Statement
    /// Descriptor](https://docs.payrexhq.com/docs/guide/developer_handbook/statement_descriptor)
    /// guide.
    pub statement_descriptor: Option<String>,

    /// The latest status of the [`PaymentIntent`]. Possible values are `awaiting_payment_method`, `awaiting_next_action`, `processing`, or `succeeded`.
    pub status: PaymentIntentStatus,

    /// If this attribute is present, it tells you what actions you need to take so that your
    /// customer can make a payment using the selected method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_action: Option<NextAction>,

    /// The URL where your customer will be redirected after completing the authentication if they
    /// didn't exit or close their browser while authenticating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,

    /// The time by which the [`PaymentIntent`] must be captured to avoid being canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_before_at: Option<Timestamp>,

    /// The time the resource was created and measured in seconds since the Unix epoch.
    pub created_at: Timestamp,

    /// The time the resource was updated and measured in seconds since the Unix epoch.
    pub updated_at: Timestamp,
}

/// All fields in this struct are optional since fields nested under billing statements have
/// optional fields. Hence, this should not be used for regular payment intent routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptionalPaymentIntent {
    /// Unique identifier for the resource. The prefix is `pi_`.
    pub id: PaymentIntentId,

    /// The amount to be collected by the [`PaymentIntent`]. This is a positive integer that your
    /// customer will pay in the smallest currency unit, cents. If the customer should pay ₱
    /// 120.50, the amount of the [`PaymentIntent`] should be 12050.
    ///
    /// The minimum amount is ₱ 20 (2000 in cents) and the maximum amount is ₱ 59,999,999.99
    /// (5999999999 in cents).
    pub amount: Option<i64>,

    /// The amount already collected by the [`PaymentIntent`]. This is a positive integer that your
    /// customer paid in the smallest currency unit, cents. If the customer paid ₱ 120.50, the
    /// `amount_received` of the [`PaymentIntent`] should be 12050.
    ///
    /// The minimum amount is ₱ 20 (2000 in cents) and the maximum amount is ₱ 59,999,999.99
    /// (5999999999 in cents).
    pub amount_received: Option<i64>,

    /// The amount that can be captured by the [`PaymentIntent`]. This is a positive integer that your
    /// customer authorized in the smallest currency unit, cents. If the customer authorized ₱
    /// 120.50, the `amount_capturable` of the [`PaymentIntent`] should be 12050.
    ///
    /// The minimum amount is ₱ 20 (2000 in cents) and the maximum amount is ₱ 59,999,999.99
    /// (5999999999 in cents).
    pub amount_capturable: Option<i64>,

    ///The client secret of this [`PaymentIntent`] used for client-side retrieval using a public API
    ///key. The client secret can be used to complete a payment from your client application.
    pub client_secret: Option<String>,

    /// A three-letter ISO currency code in uppercase. As of the moment, we only support PHP.
    pub currency: Option<Currency>,

    /// An arbitrary string attached to the [`PaymentIntent`]. Useful reference when viewing paid
    /// Payment from PayRex Dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The value is `true` if the resource's mode is live or the value is `false` if the resource mode is test.
    pub livemode: Option<bool>,

    /// A set of key-value pairs attached to the [`PaymentIntent`] and the resources created by the
    /// [`PaymentIntent`], e.g., Payment. This is useful for storing additional information about the
    /// [`PaymentIntent`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The `Payment` ID of the latest successful payment created by the [`PaymentIntent`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_payment: Option<String>,

    /// The error returned in case of a failed payment attempt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_payment_error: Option<PaymentError>,

    /// The latest `PaymentMethod` ID of attached to the [`PaymentIntent`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,

    /// The list of payment methods allowed to be processed by the [`PaymentIntent`].
    pub payment_methods: Option<Vec<String>>,

    /// A set of key-value pairs that can modify the behavior of the payment method attached to the
    /// payment intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<PaymentMethodOptions>,

    /// Text that appears on the customer's bank statement. This value overrides the merchant
    /// account's trade name. For information about requirements, including the 22-character limit,
    /// see the [Statement
    /// Descriptor](https://docs.payrexhq.com/docs/guide/developer_handbook/statement_descriptor)
    /// guide.
    pub statement_descriptor: Option<String>,

    /// The latest status of the [`PaymentIntent`]. Possible values are `awaiting_payment_method`, `awaiting_next_action`, `processing`, or `succeeded`.
    pub status: Option<PaymentIntentStatus>,

    /// If this attribute is present, it tells you what actions you need to take so that your
    /// customer can make a payment using the selected method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_action: Option<NextAction>,

    /// The URL where your customer will be redirected after completing the authentication if they
    /// didn't exit or close their browser while authenticating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,

    /// The time by which the [`PaymentIntent`] must be captured to avoid being canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_before_at: Option<Timestamp>,

    /// The time the resource was created and measured in seconds since the Unix epoch.
    pub created_at: Option<Timestamp>,

    /// The time the resource was updated and measured in seconds since the Unix epoch.
    pub updated_at: Option<Timestamp>,
}

/// The status of a [`PaymentIntent`] describes the current state of the payment process.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentStatus {
    /// Awaiting a valid payment method to be attached.
    AwaitingPaymentMethod,

    /// The payment requires a payment method.
    RequiresPaymentMethod,

    /// The payment requires confirmation before proceeding.
    RequiresConfirmation,

    /// The payment requires further action before proceeding.
    RequiresAction,

    /// The payment is being processed.
    Processing,

    /// The payment requires capture.
    RequiresCapture,

    /// The payment was cancelled.
    Canceled,

    /// The payment was successful.
    Succeeded,
}

/// Query parameters when creating a payment intent.
///
/// [Reference](https://docs.payrexhq.com/docs/api/payment_intents/create#parameters)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentIntent {
    /// The amount to be collected by the [`PaymentIntent`]. This is a positive integer your customer
    /// will pay in the smallest currency unit, cents. If the customer should pay ₱ 120.50, the
    /// amount of the [`PaymentIntent`] should be 12050.
    ///
    /// The minimum amount is ₱ 20 (2000 in cents) and the maximum amount is ₱ 59,999,999.99
    /// (5999999999 in cents).
    pub amount: i64,

    /// A three-letter ISO currency code in uppercase. As of the moment, we only support PHP.
    pub currency: Currency,

    /// The list of payment methods allowed to be processed by the [`PaymentIntent`]. Possible values
    /// are `card`, `gcash`, `maya`, and `qrph`.
    pub payment_methods: Vec<PaymentMethod>,

    /// An arbitrary string attached to the [`PaymentIntent`]. Useful reference when viewing paid
    /// Payment from PayRex Dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// A set of key-value pairs you can attach to the [`PaymentIntent`] and the resources created by
    /// the [`PaymentIntent`] e.g. Payment. This can be useful for storing additional information about
    /// the [`PaymentIntent`] in a hash format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Describes the `capture_method` of a card payment. Possible values are `automatic` or
    /// `manual`. This is used for hold then capture feature. Please refer to this
    /// [guide](https://docs.payrexhq.com/docs/guide/developer_handbook/payments/payment_methods/card/hold_then_capture)
    /// for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CaptureMethod>,

    /// A set of key-value pairs that can modify the behavior of the payment method attached to the
    /// payment intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<PaymentMethodOptions>,

    /// Text that appears on the customer's bank statement. This value overrides the merchant
    /// account's trade name. For information about requirements, including the 22-character limit,
    /// see the [Statement
    /// Descriptor](https://docs.payrexhq.com/docs/guide/developer_handbook/statement_descriptor)
    /// guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// The URL where your customer will be redirected after completing the authentication if they
    /// didn't exit or close their browser while authenticating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}

/// Query parameters when capturing a payment intent.
///
/// [Reference](https://docs.payrexhq.com/docs/api/payment_intents/capture#parameters)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturePaymentIntent {
    /// The amount to be captured by the [`PaymentIntent`]. This is a positive integer that your
    /// customer authorized in the smallest currency unit, cents. If the customer should pay ₱
    /// 120.50, the amount of the [`PaymentIntent`] should be 12050.
    ///
    /// The minimum amount is ₱ 20 (2000 in cents), and the maximum amount is ₱ 59,999,999.99
    /// (5999999999 in cents).
    pub amount: i64,
}

impl CapturePaymentIntent {
    /// Creates a new [`CapturePaymentIntent`] with the specified amount.
    #[must_use]
    pub const fn new(amount: i64) -> Self {
        Self { amount }
    }
}

impl CreatePaymentIntent {
    /// Creates a new [`CreatePaymentIntent`] with the specified amount, currency, and payment
    /// methods.
    #[must_use]
    pub fn new(amount: i64, currency: Currency, payment_methods: &[PaymentMethod]) -> Self {
        Self {
            amount,
            currency,
            payment_methods: payment_methods.to_vec(),
            description: None,
            metadata: None,
            capture_method: None,
            payment_method_options: None,
            statement_descriptor: None,
            return_url: None,
        }
    }

    /// Sets the description.
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the metadata.
    #[must_use]
    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Sets the capture method.
    #[must_use]
    pub const fn capture_method(mut self, method: CaptureMethod) -> Self {
        self.capture_method = Some(method);
        self
    }

    /// Sets the payment method options.
    #[must_use]
    pub fn payment_method_options(mut self, options: PaymentMethodOptions) -> Self {
        self.payment_method_options = Some(options);
        self
    }

    /// Sets the statement descriptor.
    #[must_use]
    pub fn statement_descriptor(mut self, descriptor: impl Into<String>) -> Self {
        self.statement_descriptor = Some(descriptor.into());
        self
    }

    /// Sets the return URL.
    #[must_use]
    pub fn return_url(mut self, url: impl Into<String>) -> Self {
        self.return_url = Some(url.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::CardOptions;

    #[test]
    fn test_create_payment_intent_builder() {
        use PaymentMethod::*;
        let payment_methods = &[Card, GCash];
        let params = CreatePaymentIntent::new(10000, Currency::PHP, payment_methods)
            .description("Test payment")
            .capture_method(CaptureMethod::Manual);

        assert_eq!(params.amount, 10000);
        assert_eq!(params.currency, Currency::PHP);
        assert_eq!(params.payment_methods, vec![Card, GCash]);
        assert_eq!(params.description, Some("Test payment".to_string()));
        assert_eq!(params.capture_method, Some(CaptureMethod::Manual));
    }

    #[test]
    fn test_create_payment_intent_with_all_options() {
        use PaymentMethod::*;
        let payment_methods = &[Card];
        let mut metadata = Metadata::new();
        metadata.insert("order_id", "12345");

        let card_options = CardOptions {
            capture_type: Some(CaptureMethod::Manual),
            allowed_bins: Some(vec!["123456".to_string()]),
            allowed_funding: Some(vec!["credit".to_string()]),
        };

        let payment_method_options = PaymentMethodOptions {
            card: Some(card_options),
        };

        let params = CreatePaymentIntent::new(10000, Currency::PHP, payment_methods)
            .description("Test payment")
            .metadata(metadata.clone())
            .capture_method(CaptureMethod::Manual)
            .payment_method_options(payment_method_options.clone())
            .statement_descriptor("TEST MERCHANT")
            .return_url("https://example.com/return");

        assert_eq!(params.amount, 10000);
        assert_eq!(params.description, Some("Test payment".to_string()));
        assert_eq!(params.metadata, Some(metadata));
        assert_eq!(params.capture_method, Some(CaptureMethod::Manual));
        assert!(params.payment_method_options.is_some());
        assert_eq!(
            params.statement_descriptor,
            Some("TEST MERCHANT".to_string())
        );
        assert_eq!(
            params.return_url,
            Some("https://example.com/return".to_string())
        );
    }

    #[test]
    fn test_capture_payment_intent() {
        let params = CapturePaymentIntent::new(5000);
        assert_eq!(params.amount, 5000);
    }

    #[test]
    fn test_payment_intent_status_serialization() {
        use serde_json;

        let status = PaymentIntentStatus::RequiresPaymentMethod;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"requires_payment_method\"");

        let status = PaymentIntentStatus::Succeeded;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"succeeded\"");
    }

    #[test]
    fn test_payment_methods_in_create_intent() {
        use PaymentMethod::*;
        use serde_json;

        let params = CreatePaymentIntent::new(10000, Currency::PHP, &[Card, GCash, Maya]);
        let json = serde_json::to_value(&params).unwrap();

        // Verify payment_methods serializes as array of strings
        let methods = json["payment_methods"].as_array().unwrap();
        assert_eq!(methods.len(), 3);
        assert_eq!(methods[0].as_str().unwrap(), "card");
        assert_eq!(methods[1].as_str().unwrap(), "gcash");
        assert_eq!(methods[2].as_str().unwrap(), "maya");
    }
}
