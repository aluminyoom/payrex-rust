//! Checkout Sessions API
//!
//! Checkout Sessions create a hosted payment page for collecting payment.

use crate::{
    Result,
    http::HttpClient,
    resources::payment_intents::PaymentIntent,
    types::{
        CheckoutSessionId, CheckoutSessionLineItemId, Currency, Metadata, PaymentMethod,
        PaymentMethodOptions, Timestamp,
    },
};
use payrex_derive::{Payrex, payrex_attr};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Checkout Sessions API
#[derive(Clone)]
pub struct CheckoutSessions {
    http: Arc<HttpClient>,
}

impl CheckoutSessions {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Creates a CheckoutSession resource.
    ///
    /// Endpoint: `POST /checkout_sessions`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/checkout_sessions/create)
    pub async fn create(&self, params: CreateCheckoutSession) -> Result<CheckoutSession> {
        self.http.post("/checkout_sessions", &params).await
    }

    /// Retrieve a CheckoutSession resource by ID.
    ///
    /// A CheckoutSession can only be retrieved from the server side using a secret API key.
    ///
    /// Endpoint: `GET /checkout_sessions/:id`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/checkout_sessions/retrieve)
    pub async fn retrieve(&self, id: &CheckoutSessionId) -> Result<CheckoutSession> {
        self.http
            .get(&format!("/checkout_sessions/{}", id.as_str()))
            .await
    }

    /// Expire a CheckoutSession resource.
    ///
    /// Endpoint: `POST /checkout_sessions/:id/expire`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/checkout_sessions/expire)
    pub async fn expire(&self, id: &CheckoutSessionId) -> Result<CheckoutSession> {
        self.http
            .post(&format!("/checkout_sessions/{}/expire", id.as_str()), &())
            .await
    }
}

/// A Checkout Session resource represents a one-time use PayRex-hosted checkout page and will
/// expire at a certain period. To learn more about PayRex Checkout, you can refer to this
/// [guide](https://docs.payrexhq.com/docs/guide/developer_handbook/payments/integrations/checkout).
#[payrex_attr(
    livemode,
    timestamp,
    metadata,
    currency = false,
    amount = true,
    description = "checkout_session"
)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckoutSession {
    /// Unique identifier for the resource. The prefix is `cs_`.
    pub id: CheckoutSessionId,

    /// A unique reference of the CheckoutSession aside from the `id` attribute. This can be an
    /// order ID, a cart ID, or similar, and can be used to reconcile the CheckoutSession with your
    /// internal system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_reference_id: Option<String>,

    /// Defines if the billing information fields will always show or managed by PayRex. Default
    /// value is `always`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details_collection: Option<String>,

    /// The client secret of the CheckoutSession.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,

    /// The latest status of the CheckoutSession. Possible values are `active`, `completed`, or `expired`.
    pub status: CheckoutSessionStatus,

    /// This attribute holds your customer's list of items to pay.
    pub line_items: Vec<CheckoutSessionLineItem>,

    /// The URL where your customer will be redirected to complete a payment.
    pub url: String,

    /// The Payment Intent resource created for the CheckoutSession.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<PaymentIntent>,

    /// The URL where your customer will be redirected after a successful payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_url: Option<String>,

    /// The URL where your customer will be redirected if they decide not to continue with the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,

    /// The list of payment methods allowed to be processed by the CheckoutSession.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<Vec<PaymentMethod>>,

    /// A set of key-value pairs that can modify the behavior of the payment method attached to the
    /// payment intent of the checkout session.
    ///
    /// To learn more about the potential values for this attribute, you can refer to the [Payment
    /// Intent](https://docs.payrexhq.com/docs/api/payment_intents) resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<PaymentMethodOptions>,

    /// This is a string that will show as the text for the pay button of the CheckoutSession. You
    /// can use this to customize the action text of the pay button. The default value is `pay`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_type: Option<String>,

    /// Text that appears on the customer's bank statement. This value overrides the merchant
    /// account's trade name. For information about requirements, including the 22-character limit,
    /// see the [Statement
    /// Descriptor](https://docs.payrexhq.com/docs/guide/developer_handbook/statement_descriptor)
    /// guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// The time when the CheckoutSession will expire. Once the CheckoutSession expires, your customer can no longer complete the payment.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Timestamp>,
}

/// The latest status of a CheckoutSession.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionStatus {
    /// The checkout session is active.
    Active,
    /// The checkout session was completed.
    Completed,

    /// The checkout session expired.
    Expired,
}

/// List of items to pay during a checkout session.
#[payrex_attr(amount = false, description = "checkout_session")]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Payrex)]
pub struct CheckoutSessionLineItem {
    /// Unique identifier for the resource. The prefix is `cs_li`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[payrex(description = "Sets the checkout session line item ID.")]
    pub id: Option<CheckoutSessionLineItemId>,

    /// The name of the line item. The name attribute describes the line item. It could be a
    /// product name or the service that you offer.
    pub name: String,

    /// The quantity of the line item. The quantity will be multiplied by the `line_item.amount` to
    /// compute the final amount of the CheckoutSession.
    pub quantity: u64,

    /// The image of the line_item. This should be a publicly accessible URL. If this is not
    /// provided, PayRex will provide a default image.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[payrex(description = "Sets the public image URL of the line item.")]
    pub image: Option<String>,
}

/// Query parameters when creating a checkout session.
///
/// [Reference](https://docs.payrexhq.com/docs/api/checkout_sessions/create#parameters)
#[payrex_attr(metadata, currency = false, description = "checkout_session")]
#[derive(Debug, Default, Clone, Serialize, Deserialize, Payrex)]
pub struct CreateCheckoutSession {
    /// A unique reference of the CheckoutSession aside from the `id` attribute. This can be an order
    /// ID, a cart ID, or similar, and can be used to reconcile the CheckoutSession with your
    /// internal system.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[payrex(description = "Sets the customer reference ID when creating a checkout session.")]
    pub customer_reference_id: Option<String>,

    /// This attribute holds your customer's list of items to pay.
    pub line_items: Vec<CheckoutSessionLineItem>,

    /// The URL where your customer will be redirected after a successful payment.
    pub success_url: String,

    /// The URL where your customer will be redirected if they decide not to continue with the payment.
    pub cancel_url: String,

    /// The list of payment methods allowed to be processed by the CheckoutSession. Plaese refer to
    /// the [allowed list of payment
    /// methods](https://docs.payrexhq.com/docs/guide/developer_handbook/payments/payment_methods)
    /// of PayRex.
    ///
    /// If this attribute is not passed, the default payment methods of your PayRex merchant account will be used.
    pub payment_methods: Vec<PaymentMethod>,

    /// A set of key-value pairs that can modify the behavior of the payment method attached to the
    /// payment intent of the checkout session.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[payrex(description = "Sets the payment method options when creating a checkout session.")]
    pub payment_method_options: Option<PaymentMethodOptions>,

    /// An epoch timestamp.
    ///
    /// The time when the CheckoutSession will expire. Once the CheckoutSession expires, your customer can no longer complete the payment.
    ///
    /// If this attribute is not passed, the CheckoutSession will expire in 24 hours.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[payrex(description = "Sets the expiration date for a checkout session during its creation.")]
    pub expires_at: Option<Timestamp>,

    /// Defines if the billing information fields will always show or managed by PayRex. Default
    /// value is `always`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[payrex(
        description = "Sets the billing details collection when creating a checkout session."
    )]
    pub billing_details_collection: Option<String>,

    /// This is a string that will show as the text for the pay button of the CheckoutSession. You can use this to customize the action text of the pay button.
    ///
    /// Default value is `pay`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[payrex(description = "Sets the submit type when creating a checkout session.")]
    pub submit_type: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        CheckoutSessionId, CheckoutSessionLineItemId, Currency, Metadata, PaymentMethod,
        PaymentMethodOptions, Timestamp,
    };
    use serde_json;

    #[test]
    fn test_checkout_session_status_serialization() {
        assert_eq!(
            serde_json::to_string(&CheckoutSessionStatus::Active).unwrap(),
            "\"active\""
        );
        assert_eq!(
            serde_json::to_string(&CheckoutSessionStatus::Completed).unwrap(),
            "\"completed\""
        );
        assert_eq!(
            serde_json::to_string(&CheckoutSessionStatus::Expired).unwrap(),
            "\"expired\""
        );
    }

    #[test]
    fn test_checkout_session_line_item_builder() {
        let item = CheckoutSessionLineItem::new("Test item", 2, 1500);
        assert_eq!(item.name, "Test item".to_string());
        assert_eq!(item.amount, 1500);
        assert_eq!(item.quantity, 2);
        assert!(item.description.is_none());
        assert!(item.image.is_none());

        let item = item.description("Desc").image("img_url");
        assert_eq!(item.description.as_deref(), Some("Desc"));
        assert_eq!(item.image.as_deref(), Some("img_url"));
    }

    #[test]
    fn test_checkout_session_line_item_serialization() {
        let mut item = CheckoutSessionLineItem::new("Test item", 2, 1500)
            .description("Desc")
            .image("img_url");
        let json = serde_json::to_value(&item).unwrap();
        assert_eq!(json["name"], "Test item");
        assert_eq!(json["amount"], 1500);
        assert_eq!(json["quantity"], 2);
        assert_eq!(json["description"], "Desc");
        assert_eq!(json["image"], "img_url");
        assert!(json.get("id").is_none());

        item.id = Some(CheckoutSessionLineItemId::new("cs_li_123"));
        let json = serde_json::to_value(&item).unwrap();
        assert_eq!(json["id"], "cs_li_123");
    }

    #[test]
    fn test_create_checkout_session_builder() {
        let line_item = CheckoutSessionLineItem::new("Item A", 1000, 1);
        let payment_methods = vec![PaymentMethod::Card];
        let params = CreateCheckoutSession::new(
            vec![line_item.clone()],
            "https://success",
            "https://cancel",
            payment_methods.clone(),
            Currency::PHP,
        );

        assert_eq!(params.currency, Currency::PHP);
        assert_eq!(params.line_items, vec![line_item]);
        assert_eq!(params.success_url, "https://success".to_string());
        assert_eq!(params.cancel_url, "https://cancel".to_string());
        assert_eq!(params.payment_methods, payment_methods);
        assert!(params.customer_reference_id.is_none());
        assert!(params.payment_method_options.is_none());
        assert!(params.expires_at.is_none());
        assert!(params.billing_details_collection.is_none());
        assert!(params.submit_type.is_none());
        assert!(params.description.is_none());
        assert!(params.metadata.is_none());
    }

    #[test]
    fn test_create_checkout_session_setters_and_serialization() {
        let line_item = CheckoutSessionLineItem::new("Item A", 1000, 1);
        let payment_methods = vec![PaymentMethod::GCash];

        let mut metadata = Metadata::new();
        metadata.insert("foo", "bar");

        let options = PaymentMethodOptions { card: None };
        let timestamp = Timestamp::from_unix(1_630_000_000);
        let params = CreateCheckoutSession::new(
            vec![line_item.clone()],
            "https://success",
            "https://cancel",
            payment_methods.clone(),
            Currency::PHP,
        )
        .customer_reference_id("cust_123")
        .expires_at(timestamp)
        .payment_method_options(options.clone())
        .billing_details_collection("always")
        .submit_type("pay")
        .description("Desc")
        .metadata(metadata.clone());

        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["customer_reference_id"], "cust_123");

        let methods = json["payment_methods"].as_array().unwrap();
        assert_eq!(methods[0].as_str().unwrap(), "gcash");
        assert_eq!(json["expires_at"], 1_630_000_000);
        assert_eq!(json["billing_details_collection"], "always");
        assert_eq!(json["submit_type"], "pay");
        assert_eq!(json["description"], "Desc");
        assert_eq!(json["metadata"]["foo"], "bar");
    }

    #[test]
    fn test_checkout_session_serialization() {
        let mut metadata = Metadata::new();
        metadata.insert("key", "value");

        let line_item = CheckoutSessionLineItem {
            id: Some(CheckoutSessionLineItemId::new("cs_li_1")),
            name: "Item".to_string(),
            amount: 1000,
            quantity: 3,
            description: Some("Desc".to_string()),
            image: Some("img".to_string()),
        };

        let session = CheckoutSession {
            id: CheckoutSessionId::new("cs_1"),
            amount: Some(1000),
            customer_reference_id: Some("cust".to_string()),
            billing_details_collection: Some("always".to_string()),
            client_secret: Some("secret".to_string()),
            status: CheckoutSessionStatus::Active,
            currency: Currency::PHP,
            line_items: vec![line_item.clone()],
            livemode: false,
            url: "http://url".to_string(),
            payment_intent: None,
            metadata: Some(metadata.clone()),
            success_url: Some("s_url".to_string()),
            cancel_url: Some("c_url".to_string()),
            payment_methods: Some(vec![PaymentMethod::Card]),
            payment_method_options: Some(PaymentMethodOptions { card: None }),
            description: Some("desc2".to_string()),
            submit_type: Some("type".to_string()),
            statement_descriptor: Some("desc3".to_string()),
            expires_at: Some(Timestamp::from_unix(123_456)),
            created_at: Timestamp::from_unix(654_321),
            updated_at: Timestamp::from_unix(654_322),
        };

        let json = serde_json::to_value(&session).unwrap();
        assert_eq!(json["id"], "cs_1");
        assert_eq!(json["amount"], 1000);
        assert_eq!(json["customer_reference_id"], "cust");
        assert_eq!(json["billing_details_collection"], "always");
        assert_eq!(json["client_secret"], "secret");
        assert_eq!(json["status"], "active");
        assert_eq!(json["currency"], "PHP");

        let items = json["line_items"].as_array().unwrap();
        assert_eq!(items[0]["id"], "cs_li_1");
        assert_eq!(items[0]["name"], "Item");
        assert_eq!(items[0]["amount"], 1000);
        assert_eq!(items[0]["quantity"], 3);
        assert_eq!(items[0]["description"], "Desc");
        assert_eq!(items[0]["image"], "img");
        assert_eq!(json["livemode"], false);
        assert_eq!(json["url"], "http://url");
        assert_eq!(json["metadata"]["key"], "value");
        assert_eq!(json["success_url"], "s_url");
        assert_eq!(json["cancel_url"], "c_url");

        let methods = json["payment_methods"].as_array().unwrap();
        assert_eq!(methods[0].as_str().unwrap(), "card");

        let opts = &json["payment_method_options"];
        assert!(opts["card"].is_null());
        assert_eq!(json["description"], "desc2");
        assert_eq!(json["submit_type"], "type");
        assert_eq!(json["statement_descriptor"], "desc3");
        assert_eq!(json["expires_at"], 123_456);
        assert_eq!(json["created_at"], 654_321);
        assert_eq!(json["updated_at"], 654_322);
    }
}
