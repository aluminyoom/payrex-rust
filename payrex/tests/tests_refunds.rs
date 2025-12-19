mod support;

use payrex::Client;
use payrex::resources::refunds::{CreateRefund, RefundReason, RefundStatus, UpdateRefund};
use payrex::types::{Currency, Metadata, PaymentId, RefundId};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockBuilder, MockServer, ResponseTemplate,
    matchers::{basic_auth, body_string_contains, method, path},
};

use crate::support::{Result, TEST_API_KEY, create_json_fixture, mock_config};

const REFUND_FIXTURE: &str = include_str!("fixtures/refund.json");

fn mock_refund_builder(method_str: &str, path_param: Option<&str>) -> MockBuilder {
    Mock::given(method(method_str))
        .and(path(format!("/refunds{}", path_param.unwrap_or(""))))
        .and(basic_auth(TEST_API_KEY, ""))
}

#[tokio::test]
async fn test_create_refund_ok() -> Result<()> {
    let mock_server = MockServer::start().await;
    let json_body = create_json_fixture(REFUND_FIXTURE);

    mock_refund_builder("POST", None)
        .and(body_string_contains(
            "payment_id=pay_M5zq1Mmun4bRZgyCXaBc4JLZm4mBtp2T",
        ))
        .and(body_string_contains("reason=requested_by_customer"))
        .and(body_string_contains("amount=167"))
        .and(body_string_contains("currency=PHP"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json_body.clone()))
        .mount(&mock_server)
        .await;

    let config = mock_config(mock_server.uri())?;
    let client = Client::with_config(config)?;
    let params = CreateRefund::new(
        PaymentId::new("pay_M5zq1Mmun4bRZgyCXaBc4JLZm4mBtp2T"),
        RefundReason::RequestedByCustomer,
        167,
        Currency::PHP,
    );
    let response = client.refunds().create(params).await?;

    assert_eq!(
        response.id,
        RefundId::new("re_o9yzYyP9iSE1nrvuE8nNxAPkMpmR5GS6")
    );
    assert_eq!(response.status, RefundStatus::Succeeded);
    Ok(())
}

#[tokio::test]
async fn test_update_refund_ok() -> Result<()> {
    let mock_server = MockServer::start().await;
    let mut json_body = create_json_fixture(REFUND_FIXTURE);
    let ref_id = "re_o9yzYyP9iSE1nrvuE8nNxAPkMpmR5GS6";
    let id = RefundId::new(ref_id);

    json_body["metadata"] = json!({
        "order_id": "order_163829"
    });

    mock_refund_builder("PUT", Some(format!("/{ref_id}").as_ref()))
        .and(body_string_contains("metadata[order_id]=order_163829"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json_body.clone()))
        .mount(&mock_server)
        .await;

    let config = mock_config(mock_server.uri())?;
    let client = Client::with_config(config)?;

    let metadata = Metadata::with_pair("order_id", "order_163829");
    let params = UpdateRefund::new().metadata(metadata.clone());
    let response = client.refunds().update(&id, params).await?;

    assert_eq!(response.id, id);
    assert_eq!(response.metadata, Some(metadata));
    Ok(())
}
