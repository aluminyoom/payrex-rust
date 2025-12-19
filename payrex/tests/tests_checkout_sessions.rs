mod support;

use payrex::Client;
use payrex::resources::checkout_sessions::{
    CheckoutSessionLineItem, CheckoutSessionStatus, CreateCheckoutSession,
};
use payrex::types::{CheckoutSessionId, Currency, PaymentMethod};
use serde_json::Value;
use wiremock::{
    Mock, MockBuilder, MockServer, ResponseTemplate,
    matchers::{basic_auth, method, path},
};

use crate::support::{Result, TEST_API_KEY, create_json_fixture, mock_config};

const CHECKOUT_SESSION_FIXTURE: &str = include_str!("fixtures/checkout-session.json");

fn mock_checkout_session_builder(method_str: &str, path_param: Option<&str>) -> MockBuilder {
    Mock::given(method(method_str))
        .and(path(format!(
            "/checkout_sessions{}",
            path_param.unwrap_or("")
        )))
        .and(basic_auth(TEST_API_KEY, ""))
}

#[tokio::test]
async fn test_create_checkout_session_ok() -> Result<()> {
    let mock_server = MockServer::start().await;
    let json_body = create_json_fixture(CHECKOUT_SESSION_FIXTURE);

    mock_checkout_session_builder("POST", None)
        .respond_with(ResponseTemplate::new(200).set_body_json(json_body.clone()))
        .mount(&mock_server)
        .await;

    let config = mock_config(mock_server.uri())?;
    let client = Client::with_config(config)?;
    let line_item = CheckoutSessionLineItem::new("Item", 1, 1000);
    let response = client
        .checkout_sessions()
        .create(CreateCheckoutSession::new(
            vec![line_item.clone()],
            "https://success",
            "https://cancel",
            vec![PaymentMethod::GCash],
            Currency::PHP,
        ))
        .await?;

    assert_eq!(
        response.id,
        CheckoutSessionId::new("cs_CuzUcxGRJ9UL3KxBSSAxhm5EhBGWMquu")
    );
    assert_eq!(response.status, CheckoutSessionStatus::Expired);
    Ok(())
}

#[tokio::test]
async fn test_retrieve_checkout_session_ok() -> Result<()> {
    let mock_server = MockServer::start().await;
    let json_body = create_json_fixture(CHECKOUT_SESSION_FIXTURE);
    let session_id = "cs_CuzUcxGRJ9UL3KxBSSAxhm5EhBGWMquu";

    mock_checkout_session_builder("GET", Some(format!("/{session_id}").as_ref()))
        .respond_with(ResponseTemplate::new(200).set_body_json(json_body.clone()))
        .mount(&mock_server)
        .await;

    let config = mock_config(mock_server.uri())?;
    let client = Client::with_config(config)?;
    let id = CheckoutSessionId::new(session_id);
    let response = client.checkout_sessions().retrieve(&id).await?;

    assert_eq!(response.id, id);
    assert_eq!(response.status, CheckoutSessionStatus::Expired);
    Ok(())
}

#[tokio::test]
async fn test_expire_checkout_session_ok() -> Result<()> {
    let mock_server = MockServer::start().await;
    let json_body = create_json_fixture(CHECKOUT_SESSION_FIXTURE);
    let session_id = "cs_CuzUcxGRJ9UL3KxBSSAxhm5EhBGWMquu";

    mock_checkout_session_builder("POST", Some(format!("/{session_id}/expire").as_ref()))
        .respond_with(ResponseTemplate::new(200).set_body_json(json_body))
        .mount(&mock_server)
        .await;

    let config = mock_config(mock_server.uri())?;
    let client = Client::with_config(config)?;
    let id = CheckoutSessionId::new(session_id);
    let response = client.checkout_sessions().expire(&id).await?;

    assert_eq!(response.id, id);
    assert_eq!(response.status, CheckoutSessionStatus::Expired);
    Ok(())
}
