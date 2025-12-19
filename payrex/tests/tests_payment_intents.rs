mod support;

use payrex::Error as PayrexError;
use payrex::resources::payment_intents::PaymentIntentStatus;
use payrex::types::PaymentIntentId;
use payrex::{
    Client, Config, ConfigBuilder,
    resources::payment_intents::CreatePaymentIntent,
    types::{Currency, PaymentMethod::*},
};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockBuilder, MockServer, ResponseTemplate,
    matchers::{basic_auth, body_string_contains, method, path, query_param},
};

use crate::support::{Result, TEST_API_KEY, create_json_fixture, mock_config};

const PAYMENT_INTENT_FIXTURE: &str = include_str!("fixtures/payment-intent.json");

fn mock_payment_intent_builder(method_str: &str, path_param: Option<&str>) -> MockBuilder {
    Mock::given(method(method_str))
        .and(path(format!(
            "/payment_intents{}",
            path_param.unwrap_or("")
        )))
        .and(basic_auth(TEST_API_KEY, ""))
}

#[tokio::test]
async fn test_create_payment_intent_ok() -> Result<()> {
    let mock_server = MockServer::start().await;
    let json_body = create_json_fixture(PAYMENT_INTENT_FIXTURE);

    mock_payment_intent_builder("POST", None)
        .and(body_string_contains("amount=10000"))
        .and(body_string_contains("currency=PHP"))
        .and(body_string_contains("payment_methods[]=card"))
        .and(body_string_contains("payment_methods[]=gcash"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json_body))
        .mount(&mock_server)
        .await;

    let config = mock_config(mock_server.uri())?;

    let client = Client::with_config(config)?;
    let params = CreatePaymentIntent::new([Card, GCash], 10_000, Currency::PHP);
    let response = client.payment_intents().create(params).await?;

    assert_eq!(response.currency, Currency::PHP);
    assert_eq!(response.amount, 10_000);
    assert_eq!(response.status, PaymentIntentStatus::AwaitingPaymentMethod);
    assert!(response.payment_methods.contains(&GCash));
    assert!(response.payment_methods.contains(&Card));

    Ok(())
}

#[tokio::test]
async fn test_retrieve_payment_intent_ok() -> Result<()> {
    let mock_server = MockServer::start().await;
    let json_body = create_json_fixture(PAYMENT_INTENT_FIXTURE);
    let payment_intent_id = "pi_FxmwbTkuQQb3qMBrgGiyNyzEFR7BKZVQ";

    mock_payment_intent_builder("GET", Some(format!("/{payment_intent_id}").as_ref()))
        .respond_with(ResponseTemplate::new(200).set_body_json(json_body))
        .mount(&mock_server)
        .await;

    let config = mock_config(mock_server.uri())?;

    let client = Client::with_config(config)?;
    let id = PaymentIntentId::new(payment_intent_id);
    let response = client.payment_intents().retrieve(&id).await?;

    assert_eq!(response.id, id);
    assert_eq!(response.currency, Currency::PHP);
    assert_eq!(response.amount, 10_000);
    assert!(response.payment_methods.contains(&GCash));
    assert!(response.payment_methods.contains(&Card));

    Ok(())
}

#[tokio::test]
async fn test_cancel_payment_intent_ok() -> Result<()> {
    let mock_server = MockServer::start().await;
    let mut json_body = create_json_fixture(PAYMENT_INTENT_FIXTURE);
    let payment_intent_id = "pi_FxmwbTkuQQb3qMBrgGiyNyzEFR7BKZVQ";
    json_body["status"] = Value::String("canceled".to_string());

    mock_payment_intent_builder(
        "POST",
        Some(format!("/{payment_intent_id}/cancel").as_ref()),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(json_body))
    .mount(&mock_server)
    .await;

    let config = mock_config(mock_server.uri())?;

    let client = Client::with_config(config)?;
    let id = PaymentIntentId::new(payment_intent_id);
    let response = client.payment_intents().cancel(&id).await?;

    assert_eq!(response.id, id);
    assert_eq!(response.status, PaymentIntentStatus::Canceled);
    assert_eq!(response.currency, Currency::PHP);
    assert_eq!(response.amount, 10_000);
    assert!(response.payment_methods.contains(&GCash));
    assert!(response.payment_methods.contains(&Card));

    Ok(())
}

// TODO: Add mock test for capturing payment intents
