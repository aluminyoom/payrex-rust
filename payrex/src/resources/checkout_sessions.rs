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
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct CheckoutSessions {
    http: Arc<HttpClient>,
}

impl CheckoutSessions {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub async fn create(&self, params: CreateCheckoutSession) -> Result<CheckoutSession> {
        self.http.post("/checkout_sessions", &params).await
    }

    pub async fn retrieve(&self, id: &CheckoutSessionId) -> Result<CheckoutSession> {
        self.http
            .get(&format!("/checkout_sessions/{}", id.as_str()))
            .await
    }

    pub async fn expire(&self, id: &CheckoutSessionId) -> Result<CheckoutSession> {
        self.http
            .post(&format!("/checkout_sessions/{}/expire", id.as_str()), &())
            .await
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckoutSession {
    pub id: CheckoutSessionId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details_collection: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    pub status: CheckoutSessionStatus,
    pub currency: Currency,
    pub line_items: Vec<CheckoutSessionLineItem>,
    pub livemode: bool,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<PaymentIntent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<Vec<PaymentMethod>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<PaymentMethodOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Timestamp>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionStatus {
    Active,
    Completed,
    Expired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckoutSessionLineItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<CheckoutSessionLineItemId>,
    pub name: String,
    pub amount: u64,
    pub quantity: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCheckoutSession {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_reference_id: Option<String>,
    pub currency: Currency,
    pub line_items: Vec<CheckoutSessionLineItem>,
    pub success_url: String,
    pub cancel_url: String,
    pub payment_methods: Vec<PaymentMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<PaymentMethodOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details_collection: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl CreateCheckoutSession {
    #[must_use]
    pub fn new(
        currency: Currency,
        line_items: Vec<CheckoutSessionLineItem>,
        success_url: impl Into<String>,
        cancel_url: impl Into<String>,
        payment_methods: Vec<PaymentMethod>,
    ) -> Self {
        Self {
            customer_reference_id: None,
            currency,
            line_items,
            success_url: success_url.into(),
            cancel_url: cancel_url.into(),
            payment_methods,
            payment_method_options: None,
            expires_at: None,
            billing_details_collection: None,
            submit_type: None,
            description: None,
            metadata: None,
        }
    }

    pub fn customer_reference_id(mut self, id: impl Into<String>) -> Self {
        self.customer_reference_id = Some(id.into());
        self
    }

    pub fn expires_at(mut self, timestamp: Timestamp) -> Self {
        self.expires_at = Some(timestamp);
        self
    }

    pub fn payment_method_options(mut self, options: PaymentMethodOptions) -> Self {
        self.payment_method_options = Some(options);
        self
    }

    pub fn billing_details_collection(mut self, collection: impl Into<String>) -> Self {
        self.billing_details_collection = Some(collection.into());
        self
    }

    pub fn submit_type(mut self, submit_type: impl Into<String>) -> Self {
        self.submit_type = Some(submit_type.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

impl CheckoutSessionLineItem {
    #[must_use]
    pub fn new(name: impl Into<String>, amount: u64, quantity: u64) -> Self {
        Self {
            id: None,
            name: name.into(),
            amount,
            quantity,
            description: None,
            image: None,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn image(mut self, image: impl Into<String>) -> Self {
        self.image = Some(image.into());
        self
    }
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
        let item = CheckoutSessionLineItem::new("Test item", 1500, 2);
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
        let mut item = CheckoutSessionLineItem::new("Test item", 1500, 2)
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
            Currency::PHP,
            vec![line_item.clone()],
            "https://success",
            "https://cancel",
            payment_methods.clone(),
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
            Currency::PHP,
            vec![line_item.clone()],
            "https://success",
            "https://cancel",
            payment_methods.clone(),
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
