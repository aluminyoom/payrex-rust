mod support;

use payrex::Client;
use payrex::resources::webhooks::{CreateWebhook, UpdateWebhook, WebhookListParams, WebhookStatus};
use payrex::types::WebhookId;
use payrex::types::event::{BillingStatementEvent, CheckoutSessionEvent, EventType};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockBuilder, MockServer, ResponseTemplate,
    matchers::{basic_auth, method, path, query_param},
};

use crate::support::{Result, TEST_API_KEY, create_json_fixture, mock_config};

const WEBHOOK_FIXTURE: &str = include_str!("fixtures/webhook.json");
const WEBHOOK_LIST_FIXTURE: &str = include_str!("fixtures/webhook-list.json");

fn mock_webhook_builder(method_str: &str, path_param: Option<&str>) -> MockBuilder {
    Mock::given(method(method_str))
        .and(path(format!("/webhooks{}", path_param.unwrap_or(""))))
        .and(basic_auth(TEST_API_KEY, ""))
}

#[tokio::test]
async fn test_create_webhook_ok() -> Result<()> {
    let mock_server = MockServer::start().await;
    let json_body = create_json_fixture(WEBHOOK_FIXTURE);

    mock_webhook_builder("POST", None)
        .respond_with(ResponseTemplate::new(200).set_body_json(json_body.clone()))
        .mount(&mock_server)
        .await;

    let config = mock_config(mock_server.uri())?;
    let client = Client::with_config(config)?;

    let billing_statement_created = EventType::BillingStatement(BillingStatementEvent::Created);
    let params = CreateWebhook::new(
        "https://testpayrexwebhooks.com".to_string(),
        vec![billing_statement_created.clone()],
    );
    let response = client.webhooks().create(params).await?;

    assert_eq!(
        response.id,
        WebhookId::new("wh_pDQvyHKPJ5J5SahAjDYLG2u6TUxvB6GH")
    );
    assert_eq!(response.url, "https://testpayrexwebhooks.com".to_string());
    assert_eq!(response.status, WebhookStatus::Enabled);
    assert!(response.events.contains(&billing_statement_created));

    Ok(())
}

#[tokio::test]
async fn test_retrieve_webhook_ok() -> Result<()> {
    let mock_server = MockServer::start().await;
    let json_body = create_json_fixture(WEBHOOK_FIXTURE);
    let wh_id = "wh_pDQvyHKPJ5J5SahAjDYLG2u6TUxvB6GH";

    mock_webhook_builder("GET", Some(format!("/{wh_id}").as_ref()))
        .respond_with(ResponseTemplate::new(200).set_body_json(json_body.clone()))
        .mount(&mock_server)
        .await;

    let config = mock_config(mock_server.uri())?;
    let client = Client::with_config(config)?;
    let id = WebhookId::new(wh_id);
    let response = client.webhooks().retrieve(&id).await?;

    assert_eq!(response.id, id);
    Ok(())
}

#[tokio::test]
async fn test_update_webhook_ok() -> Result<()> {
    let mock_server = MockServer::start().await;
    let mut json_body = create_json_fixture(WEBHOOK_FIXTURE);
    let wh_id = "wh_pDQvyHKPJ5J5SahAjDYLG2u6TUxvB6GH";

    json_body["url"] = Value::String("https://new-url.com".to_string());

    mock_webhook_builder("PUT", Some(format!("/{wh_id}").as_ref()))
        .respond_with(ResponseTemplate::new(200).set_body_json(json_body.clone()))
        .mount(&mock_server)
        .await;

    let config = mock_config(mock_server.uri())?;
    let client = Client::with_config(config)?;
    let id = WebhookId::new(wh_id);
    let params = UpdateWebhook::new().url("https://new-url.com".to_string());
    let response = client.webhooks().update(&id, params).await?;

    assert_eq!(response.id, id);
    assert_eq!(response.url, "https://new-url.com");
    Ok(())
}

#[tokio::test]
async fn test_delete_webhook_ok() -> Result<()> {
    let mock_server = MockServer::start().await;
    let wh_id = "wh_pDQvyHKPJ5J5SahAjDYLG2u6TUxvB6GH";
    let json_body = json!({
        "id": "wh_pDQvyHKPJ5J5SahAjDYLG2u6TUxvB6GH",
        "resource": "webhook",
        "deleted": true
    });

    mock_webhook_builder("DELETE", Some(format!("/{wh_id}").as_ref()))
        .respond_with(ResponseTemplate::new(200).set_body_json(json_body))
        .mount(&mock_server)
        .await;

    let config = mock_config(mock_server.uri())?;
    let client = Client::with_config(config)?;
    let id = WebhookId::new(wh_id);

    let response = client.webhooks().delete(&id).await?;

    assert_eq!(response.id, id);
    assert!(response.deleted);

    Ok(())
}

#[tokio::test]
async fn test_list_webhook_ok() -> Result<()> {
    let mock_server = MockServer::start().await;
    let json_body = create_json_fixture(WEBHOOK_LIST_FIXTURE);

    mock_webhook_builder("GET", None)
        .respond_with(ResponseTemplate::new(200).set_body_json(json_body.clone()))
        .mount(&mock_server)
        .await;

    let config = mock_config(mock_server.uri())?;
    let client = Client::with_config(config)?;
    let params = WebhookListParams::new();
    let list = client.webhooks().list(params).await?;

    assert!(!list.data.is_empty());
    assert_eq!(
        list.data[0].id,
        WebhookId::new("wh_pDQvyHKPJ5J5SahAjDYLG2u6TUxvB6GH")
    );
    Ok(())
}

#[tokio::test]
async fn test_enable_disable_webhook_ok() -> Result<()> {
    let mock_server = MockServer::start().await;
    let json_body = create_json_fixture(WEBHOOK_FIXTURE);
    let wh_id = "wh_pDQvyHKPJ5J5SahAjDYLG2u6TUxvB6GH";

    // Enable
    mock_webhook_builder("POST", Some(format!("/{wh_id}/enable").as_ref()))
        .respond_with(ResponseTemplate::new(200).set_body_json(json_body.clone()))
        .mount(&mock_server)
        .await;
    // Disable
    mock_webhook_builder("POST", Some(format!("/{wh_id}/disable").as_ref()))
        .respond_with(ResponseTemplate::new(200).set_body_json(json_body.clone()))
        .mount(&mock_server)
        .await;

    let config = mock_config(mock_server.uri())?;
    let client = Client::with_config(config)?;
    let id = WebhookId::new(wh_id);
    let resp_enable = client.webhooks().enable(&id).await?;
    let resp_disable = client.webhooks().disable(&id).await?;

    assert_eq!(resp_enable.id, id);
    assert_eq!(resp_disable.id, id);
    Ok(())
}
