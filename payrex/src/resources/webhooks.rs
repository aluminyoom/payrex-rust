//! Webhooks API
//!
//! Webhooks allow you to receive real-time notifications about events.

use crate::{
    Result,
    http::HttpClient,
    types::{List, ListParams, Timestamp, WebhookId, event::EventType},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct Webhooks {
    http: Arc<HttpClient>,
}

impl Webhooks {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub async fn create(&self, params: CreateWebhook) -> Result<Webhook> {
        self.http.post("/webhooks", &params).await
    }

    pub async fn retrieve(&self, id: &WebhookId) -> Result<Webhook> {
        self.http.get(&format!("/webhooks/{}", id.as_str())).await
    }

    pub async fn update(&self, id: &WebhookId, params: UpdateWebhook) -> Result<Webhook> {
        self.http
            .put(&format!("/webhooks/{}", id.as_str()), &params)
            .await
    }

    pub async fn delete(&self, id: &WebhookId) -> Result<()> {
        self.http
            .delete(&format!("/webhooks/{}", id.as_str()))
            .await
    }

    pub async fn list(&self, params: WebhookListParams) -> Result<List<Webhook>> {
        self.http.get_with_params("/webhooks", &params).await
    }

    pub async fn enable(&self, id: &WebhookId) -> Result<Webhook> {
        self.http
            .post(&format!("/webhooks/{}/enable", id.as_str()), &())
            .await
    }

    pub async fn disable(&self, id: &WebhookId) -> Result<Webhook> {
        self.http
            .post(&format!("/webhooks/{}/disable", id.as_str()), &())
            .await
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Webhook {
    pub id: WebhookId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
    pub status: WebhookStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub livemode: bool,
    pub url: String,
    pub events: Vec<EventType>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebhookStatus {
    Enabled,
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWebhook {
    pub url: String,
    pub events: Vec<EventType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateWebhook {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<EventType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebhookListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ListParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl CreateWebhook {
    #[must_use]
    pub fn new(url: impl Into<String>, events: Vec<EventType>) -> Self {
        Self {
            url: url.into(),
            events,
            description: None,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

impl UpdateWebhook {
    #[must_use]
    pub fn new() -> Self {
        Self {
            url: None,
            events: None,
            description: None,
        }
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn events(mut self, events: Vec<EventType>) -> Self {
        self.events = Some(events);
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
