mod support;

use payrex::Error as PayrexError;
use payrex::resources::payment_intents::PaymentIntentStatus;
use payrex::resources::payments::{PaymentStatus, UpdatePayment};
use payrex::types::{Metadata, PaymentId, PaymentIntentId, PaymentMethod, metadata};
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

const PAYMENT_FIXTURE: &str = include_str!("fixtures/payment.json");

fn mock_payment_builder(method_str: &str, path_param: Option<&str>) -> MockBuilder {
    Mock::given(method(method_str))
        .and(path(format!("/payments{}", path_param.unwrap_or(""))))
        .and(basic_auth(TEST_API_KEY, ""))
}

#[tokio::test]
async fn test_retrieve_payment_ok() -> Result<()> {
    let mock_server = MockServer::start().await;
    let json_body = create_json_fixture(PAYMENT_FIXTURE);
    let payment_id = "pay_M5zq1Mmun4bRZgyCXaBc4JLZm4mBtp2T";

    mock_payment_builder("GET", Some(format!("/{payment_id}").as_ref()))
        .respond_with(ResponseTemplate::new(200).set_body_json(json_body))
        .mount(&mock_server)
        .await;

    let config = mock_config(mock_server.uri())?;

    let client = Client::with_config(config)?;
    let id = PaymentId::new(payment_id);
    let response = client.payments().retrieve(&id).await?;

    assert_eq!(response.id, id);
    assert_eq!(response.currency, Currency::PHP);
    assert_eq!(response.amount, 52_500);
    assert_eq!(response.amount_refunded, 525);
    assert_eq!(response.fee, 1050);
    assert_eq!(response.status, PaymentStatus::Paid);
    assert_eq!(response.payment_method.method_type, PaymentMethod::Maya);

    Ok(())
}

#[tokio::test]
async fn test_update_payment_ok() -> Result<()> {
    let mock_server = MockServer::start().await;
    let mut json_body = create_json_fixture(PAYMENT_FIXTURE);
    let payment_id = "pay_M5zq1Mmun4bRZgyCXaBc4JLZm4mBtp2T";

    json_body["description"] = Value::String("New description".to_string());
    json_body["metadata"] = json!({
        "order_id": "order_238afec81"
    });

    mock_payment_builder("PUT", Some(format!("/{payment_id}").as_ref()))
        .and(body_string_contains("description=New+description"))
        .and(body_string_contains("metadata[order_id]=order_238afec81"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json_body))
        .mount(&mock_server)
        .await;

    let config = mock_config(mock_server.uri())?;
    let client = Client::with_config(config)?;

    let id = PaymentId::new(payment_id);
    let metadata = Metadata::with_pair("order_id", "order_238afec81");
    let params = UpdatePayment::new()
        .metadata(metadata.clone())
        .description("New description");

    let response = client.payments().update(&id, params).await?;

    assert_eq!(response.id, id);
    assert_eq!(response.description, Some("New description".to_string()));
    assert_eq!(response.metadata, Some(metadata));

    Ok(())
}
