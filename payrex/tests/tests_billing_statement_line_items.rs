mod support;

use payrex::Client;
use payrex::resources::billing_statement_line_items::{
    CreateBillingStatementLineItem, UpdateBillingStatementLineItem,
};
use payrex::types::{BillingStatementId, BillingStatementLineItemId};
use serde_json::{Number, Value};
use wiremock::{
    Mock, MockBuilder, MockServer, ResponseTemplate,
    matchers::{basic_auth, body_string_contains, method, path},
};

use crate::support::{Result, TEST_API_KEY, create_json_fixture, mock_config};

const LINE_ITEM_FIXTURE: &str = include_str!("fixtures/billing-statement-line-item.json");
const ERR_LINE_ITEM_FIXTURE: &str =
    include_str!("fixtures/err-put-billing-statement-line-item.json");

fn mock_line_item_builder(method_str: &str, path_param: Option<&str>) -> MockBuilder {
    Mock::given(method(method_str))
        .and(path(format!(
            "/billing_statement_line_items{}",
            path_param.unwrap_or("")
        )))
        .and(basic_auth(TEST_API_KEY, ""))
}

#[tokio::test]
async fn test_create_billing_statement_line_item_ok() -> Result<()> {
    let mock_server = MockServer::start().await;
    let json_body = create_json_fixture(LINE_ITEM_FIXTURE);
    let stmt_id = BillingStatementId::new("bstm_f4rdf8645sMBn44osn2ttXgrM8FnUT5U");

    mock_line_item_builder("POST", None)
        .respond_with(ResponseTemplate::new(200).set_body_json(json_body.clone()))
        .mount(&mock_server)
        .await;

    let config = mock_config(mock_server.uri())?;
    let client = Client::with_config(config)?;
    let params = CreateBillingStatementLineItem::new(stmt_id.clone(), 72000, 1);
    let response = client.billing_statement_line_items().create(params).await?;

    assert_eq!(
        response.id,
        BillingStatementLineItemId::new("bstm_li_etTeXvtDUVQnE86m3cUQ7Y6AzgKMGpur")
    );
    assert_eq!(response.unit_price, 72000);
    assert_eq!(response.quantity, 1);
    Ok(())
}

#[tokio::test]
async fn test_update_billing_statement_line_item_ok() -> Result<()> {
    let mock_server = MockServer::start().await;
    let mut json_body = create_json_fixture(LINE_ITEM_FIXTURE);
    let item_id = BillingStatementLineItemId::new("bstm_li_etTeXvtDUVQnE86m3cUQ7Y6AzgKMGpur");

    json_body["unit_price"] = Value::Number(Number::from(80000));
    json_body["quantity"] = Value::Number(Number::from(2));

    mock_line_item_builder("PUT", Some(format!("/{item_id}").as_ref()))
        .and(body_string_contains("unit_price=80000"))
        .and(body_string_contains("quantity=2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json_body.clone()))
        .mount(&mock_server)
        .await;

    let config = mock_config(mock_server.uri())?;
    let client = Client::with_config(config)?;
    let params = UpdateBillingStatementLineItem::new()
        .unit_price(80000)
        .quantity(2);
    let response = client
        .billing_statement_line_items()
        .update(item_id.clone(), params)
        .await?;

    assert_eq!(response.unit_price, 80000);
    assert_eq!(response.quantity, 2);
    Ok(())
}

#[tokio::test]
async fn test_update_billing_statement_line_item_error() -> Result<()> {
    let mock_server = MockServer::start().await;
    let json_body = create_json_fixture(ERR_LINE_ITEM_FIXTURE);
    let item_id = BillingStatementLineItemId::new("bstm_li_etTeXvtDUVQnE86m3cUQ7Y6AzgKMGpur");

    mock_line_item_builder("PUT", Some(format!("/{item_id}").as_ref()))
        .respond_with(ResponseTemplate::new(400).set_body_json(json_body.clone()))
        .mount(&mock_server)
        .await;

    let config = mock_config(mock_server.uri())?;
    let client = Client::with_config(config)?;
    let result = client
        .billing_statement_line_items()
        .update(item_id, UpdateBillingStatementLineItem::new())
        .await;

    let err = result.unwrap_err();
    assert_eq!(err.status_code(), Some(400));
    assert!(err.to_string().contains("resource_invalid_state"));
    Ok(())
}
