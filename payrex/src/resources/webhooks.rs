//! Webhooks API
//!
//! Webhooks allow you to receive real-time notifications about events.

use crate::{
    Result,
    http::HttpClient,
    types::{Deleted, List, ListParams, Timestamp, WebhookId, event::EventType},
};
use payrex_derive::{Payrex, payrex_attr};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Webhooks API
#[derive(Clone)]
pub struct Webhooks {
    http: Arc<HttpClient>,
}

impl Webhooks {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Creates a Webhook resource.
    ///
    /// Endpoint: `POST /webhooks`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/webhooks/create)
    pub async fn create(&self, params: CreateWebhook) -> Result<Webhook> {
        self.http.post("/webhooks", &params).await
    }

    /// Retrieve a Webhook resource by ID.
    ///
    /// Endpoint: `GET /webhooks/:id`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/webhooks/retrieve)
    pub async fn retrieve(&self, id: &WebhookId) -> Result<Webhook> {
        self.http.get(&format!("/webhooks/{}", id.as_str())).await
    }

    /// Updates a Webhook resource.
    ///
    /// Endpoint: `PUT /webhooks/:id`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/webhooks/update)
    pub async fn update(&self, id: &WebhookId, params: UpdateWebhook) -> Result<Webhook> {
        self.http
            .put(&format!("/webhooks/{}", id.as_str()), &params)
            .await
    }

    /// Delete a Webhook resource by ID.
    ///
    /// Endpoint: `DELETE /webhooks/:id`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/webhooks/delete)
    pub async fn delete(&self, id: &WebhookId) -> Result<Deleted<WebhookId>> {
        self.http
            .delete(&format!("/webhooks/{}", id.as_str()))
            .await
    }

    /// List Webhook resources.
    ///
    /// Endpoint: `GET /webhooks`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/webhooks/list)
    pub async fn list(&self, params: WebhookListParams) -> Result<List<Webhook>> {
        self.http.get_with_params("/webhooks", &params).await
    }

    /// Enable a Webhook resource by ID.
    ///
    /// Endpoint: `POST /webhooks/:id/enable`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/webhooks/enable)
    pub async fn enable(&self, id: &WebhookId) -> Result<Webhook> {
        self.http
            .post(&format!("/webhooks/{}/enable", id.as_str()), &())
            .await
    }

    /// Disable a Webhook resource by ID.
    ///
    /// Endpoint: `POST /webhooks/:id/disable`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/webhooks/disable)
    pub async fn disable(&self, id: &WebhookId) -> Result<Webhook> {
        self.http
            .post(&format!("/webhooks/{}/disable", id.as_str()), &())
            .await
    }
}

/// A Webhook resource is used to notify your application about events in your PayRex account.
///
/// To learn more about webhooks, please refer to this
/// [guide](https://docs.payrexhq.com/docs/guide/developer_handbook/webhooks).
#[payrex_attr(livemode, timestamp, description = "webhook")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Webhook {
    /// Unique identifier for the resource. The prefix is `wh_`.
    pub id: WebhookId,

    /// The secret_key is used for webhook signature verification.
    ///
    /// To know more about webhook signature verification, please refer to this
    /// [guide](https://docs.payrexhq.com/docs/guide/developer_handbook/webhooks#4-secure-your-webhook-by-implementing-webhook-signature-verification).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,

    /// The latest status of the Webhook. Possible values are `enabled` or `disabled`. A disabled
    /// webhook means future events and events with remaining retries that the webhook should send
    /// will discontinue.
    pub status: WebhookStatus,

    /// The URL where the webhook will send the event. To improve the security of the webhook, your
    /// URL should use HTTPS.
    pub url: String,

    /// An array of strings that defines the list of events the webhook will listen to.
    pub events: Vec<EventType>,
}

/// The latest status of a Webhook.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebhookStatus {
    /// Webhook is enabled.
    Enabled,

    /// Webhook is disabled.
    Disabled,
}

/// Query parameters when creating a webhook.
///
/// [Reference](https://docs.payrexhq.com/docs/api/webhooks/create#parameters)
#[payrex_attr(description = "webhook")]
#[derive(Debug, Default, Clone, Serialize, Deserialize, Payrex)]
pub struct CreateWebhook {
    /// The URL where PayRex will send the event that happened from your account. For security
    /// purposes, the URL must be using HTTPS protocol.
    pub url: String,

    /// An array of strings that defines the list of events the webhook will listen to. To learn
    /// about the possible values, please refer to this
    /// [list](https://docs.payrexhq.com/docs/api/events/event_types).
    pub events: Vec<EventType>,
}

/// Query parameters when updating a webhook.
///
/// [Reference](https://docs.payrexhq.com/docs/api/webhooks/update#parameters)
#[payrex_attr(description = "webhook")]
#[derive(Debug, Default, Clone, Serialize, Deserialize, Payrex)]
pub struct UpdateWebhook {
    /// The URL where the webhook will send the event. For security purposes, the URL must be using HTTPS protocol.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[payrex(description = "Sets the URL in the query params when updating a webhook.")]
    pub url: Option<String>,

    /// An array of strings that defines the list of events the webhook will listen to. To learn
    /// about the possible values, please refer to this
    /// [list](https://docs.payrexhq.com/docs/api/events/event_types).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[payrex(
        description = "Sets the list of events in the query params when updating a webhook. Note that this overrides existing events to listen to in the webhook."
    )]
    pub events: Option<Vec<EventType>>,
}

/// Query parameters when listing webhook resources.
///
/// [Reference](https://docs.payrexhq.com/docs/api/webhooks/list#parameters)
#[payrex_attr(description = "webhook")]
#[derive(Debug, Default, Clone, Serialize, Deserialize, Payrex)]
pub struct WebhookListParams {
    /// Baseline pagination fields such as `limit`, `before`, and `after`.
    #[serde(flatten)]
    pub list_params: ListParams,

    /// You can search your webhooks via `url`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[payrex(description = "Sets the URL in the query params when listing webhooks.")]
    pub url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::event::CheckoutSessionEvent;
    use serde_json;

    #[test]
    fn test_webhook_status_serialization() {
        assert_eq!(
            serde_json::to_string(&WebhookStatus::Enabled).unwrap(),
            "\"enabled\""
        );
        assert_eq!(
            serde_json::to_string(&WebhookStatus::Disabled).unwrap(),
            "\"disabled\""
        );
    }

    #[test]
    fn test_webhook_serialization() {
        let webhook = Webhook {
            id: WebhookId::new("wh_123"),
            secret_key: Some("secret".to_string()),
            status: WebhookStatus::Enabled,
            description: Some("desc".to_string()),
            livemode: false,
            url: "http://url".to_string(),
            events: vec![EventType::CheckoutSession(CheckoutSessionEvent::Expired)],
            created_at: Timestamp::from_unix(1_600_000),
            updated_at: Timestamp::from_unix(1_600_001),
        };

        let json = serde_json::to_value(&webhook).unwrap();
        assert_eq!(json["id"], "wh_123");
        assert_eq!(json["secret_key"], "secret");
        assert_eq!(json["status"], "enabled");
        assert_eq!(json["description"], "desc");
        assert_eq!(json["livemode"], false);
        assert_eq!(json["url"], "http://url");

        let events = json["events"].as_array().unwrap();
        assert_eq!(events[0].as_str().unwrap(), "checkout_session.expired");
        assert_eq!(json["created_at"], 1_600_000);
        assert_eq!(json["updated_at"], 1_600_001);
    }

    #[test]
    fn test_create_webhook_builder() {
        let events = vec![EventType::CheckoutSession(CheckoutSessionEvent::Expired)];
        let params = CreateWebhook::new("https://example.com", events.clone());

        assert_eq!(params.url, "https://example.com".to_string());
        assert_eq!(params.events, events);
        assert!(params.description.is_none());
    }

    #[test]
    fn test_create_webhook_serialization() {
        let events = vec![EventType::CheckoutSession(CheckoutSessionEvent::Expired)];
        let params = CreateWebhook::new("https://example.com", events.clone()).description("desc");

        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["url"], "https://example.com");

        let evs = json["events"].as_array().unwrap();
        assert_eq!(evs[0].as_str().unwrap(), "checkout_session.expired");
        assert_eq!(json["description"], "desc");
    }

    #[test]
    fn test_update_webhook_builder() {
        let events = vec![EventType::CheckoutSession(CheckoutSessionEvent::Expired)];
        let params = UpdateWebhook::new()
            .url("https://example.com")
            .events(events.clone())
            .description("desc");

        assert_eq!(params.url.as_deref(), Some("https://example.com"));
        assert_eq!(params.events, Some(events));
        assert_eq!(params.description.as_deref(), Some("desc"));
    }

    #[test]
    fn test_update_webhook_serialization() {
        let serialized_empty = serde_json::to_string(&UpdateWebhook::new()).unwrap();
        assert_eq!(serialized_empty, "{}");

        let events = vec![EventType::CheckoutSession(CheckoutSessionEvent::Expired)];
        let params = UpdateWebhook::new()
            .url("https://example.com")
            .events(events.clone())
            .description("desc");

        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["url"], "https://example.com");

        let evs = json["events"].as_array().unwrap();
        assert_eq!(evs[0].as_str().unwrap(), "checkout_session.expired");
        assert_eq!(json["description"], "desc");
    }
}
