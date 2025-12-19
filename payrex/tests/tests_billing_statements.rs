mod support;

use payrex::Client;
use payrex::resources::billing_statements::{CreateBillingStatement, UpdateBillingStatement};
use payrex::types::{BillingStatementId, Currency, CustomerId};
use serde_json::Value;
use wiremock::{
    Mock, MockBuilder, MockServer, ResponseTemplate,
    matchers::{basic_auth, method, path},
};

use crate::support::{Result, TEST_API_KEY, create_json_fixture, mock_config};

const BILLING_STATEMENT_FIXTURE: &str = include_str!("fixtures/billing-statement.json");

fn mock_billing_statement_builder(method_str: &str, path_param: Option<&str>) -> MockBuilder {
    Mock::given(method(method_str))
        .and(path(format!(
            "/billing_statements{}",
            path_param.unwrap_or("")
        )))
        .and(basic_auth(TEST_API_KEY, ""))
}

#[tokio::test]
async fn test_create_billing_statement_ok() -> Result<()> {
    let mock_server = MockServer::start().await;
    let json_body = create_json_fixture(BILLING_STATEMENT_FIXTURE);

    mock_billing_statement_builder("POST", None)
        .respond_with(ResponseTemplate::new(200).set_body_json(json_body.clone()))
        .mount(&mock_server)
        .await;

    let config = mock_config(mock_server.uri())?;
    let client = Client::with_config(config)?;
    let params = CreateBillingStatement::new(CustomerId::new("cus_123"), Currency::PHP);
    let response = client.billing_statements().create(params).await?;

    assert_eq!(
        response.id,
        BillingStatementId::new("bstm_f4rdf8645sMBn44osn2ttXgrM8FnUT5U")
    );
    Ok(())
}

#[tokio::test]
async fn test_retrieve_billing_statement_ok() -> Result<()> {
    let mock_server = MockServer::start().await;
    let json_body = create_json_fixture(BILLING_STATEMENT_FIXTURE);
    let stmt_id = "bstm_f4rdf8645sMBn44osn2ttXgrM8FnUT5U";

    mock_billing_statement_builder("GET", Some(format!("/{stmt_id}").as_ref()))
        .respond_with(ResponseTemplate::new(200).set_body_json(json_body.clone()))
        .mount(&mock_server)
        .await;

    let config = mock_config(mock_server.uri())?;
    let client = Client::with_config(config)?;
    let id = BillingStatementId::new(stmt_id);
    let response = client.billing_statements().retrieve(&id).await?;

    assert_eq!(response.id, id);
    Ok(())
}

#[tokio::test]
async fn test_update_billing_statement_ok() -> Result<()> {
    let mock_server = MockServer::start().await;
    let json_body = create_json_fixture(BILLING_STATEMENT_FIXTURE);
    let stmt_id = "bstm_f4rdf8645sMBn44osn2ttXgrM8FnUT5U";

    mock_billing_statement_builder("PUT", Some(format!("/{stmt_id}").as_ref()))
        .respond_with(ResponseTemplate::new(200).set_body_json(json_body.clone()))
        .mount(&mock_server)
        .await;

    let config = mock_config(mock_server.uri())?;
    let client = Client::with_config(config)?;
    let id = BillingStatementId::new(stmt_id);
    let params = UpdateBillingStatement::new();
    let response = client.billing_statements().update(&id, params).await?;

    assert_eq!(response.id, id);
    Ok(())
}

#[tokio::test]
async fn test_finalize_billing_statement_ok() -> Result<()> {
    let mock_server = MockServer::start().await;
    let json_body = create_json_fixture(BILLING_STATEMENT_FIXTURE);
    let stmt_id = "bstm_f4rdf8645sMBn44osn2ttXgrM8FnUT5U";

    mock_billing_statement_builder("POST", Some(format!("/{stmt_id}/finalize").as_ref()))
        .respond_with(ResponseTemplate::new(200).set_body_json(json_body.clone()))
        .mount(&mock_server)
        .await;

    let config = mock_config(mock_server.uri())?;
    let client = Client::with_config(config)?;
    let id = BillingStatementId::new(stmt_id);
    let response = client.billing_statements().finalize(&id).await?;

    assert_eq!(response.id, id);
    Ok(())
}

#[tokio::test]
async fn test_send_billing_statement_ok() -> Result<()> {
    let mock_server = MockServer::start().await;
    let json_body = create_json_fixture(BILLING_STATEMENT_FIXTURE);
    let stmt_id = "bstm_f4rdf8645sMBn44osn2ttXgrM8FnUT5U";

    mock_billing_statement_builder("POST", Some(format!("/{stmt_id}/send").as_ref()))
        .respond_with(ResponseTemplate::new(200).set_body_json(json_body.clone()))
        .mount(&mock_server)
        .await;

    let config = mock_config(mock_server.uri())?;
    let client = Client::with_config(config)?;
    let id = BillingStatementId::new(stmt_id);
    let response = client.billing_statements().send(&id).await?;

    assert_eq!(response.id, id);
    Ok(())
}

#[tokio::test]
async fn test_void_billing_statement_ok() -> Result<()> {
    let mock_server = MockServer::start().await;
    let json_body = create_json_fixture(BILLING_STATEMENT_FIXTURE);
    let stmt_id = "bstm_f4rdf8645sMBn44osn2ttXgrM8FnUT5U";

    mock_billing_statement_builder("POST", Some(format!("/{stmt_id}/void").as_ref()))
        .respond_with(ResponseTemplate::new(200).set_body_json(json_body.clone()))
        .mount(&mock_server)
        .await;

    let config = mock_config(mock_server.uri())?;
    let client = Client::with_config(config)?;
    let id = BillingStatementId::new(stmt_id);
    let response = client.billing_statements().void(&id).await?;

    assert_eq!(response.id, id);
    Ok(())
}

#[tokio::test]
async fn test_mark_uncollectible_billing_statement_ok() -> Result<()> {
    let mock_server = MockServer::start().await;
    let json_body = create_json_fixture(BILLING_STATEMENT_FIXTURE);
    let stmt_id = "bstm_f4rdf8645sMBn44osn2ttXgrM8FnUT5U";

    mock_billing_statement_builder(
        "POST",
        Some(format!("/{stmt_id}/mark_uncollectible").as_ref()),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(json_body))
    .mount(&mock_server)
    .await;

    let config = mock_config(mock_server.uri())?;
    let client = Client::with_config(config)?;
    let id = BillingStatementId::new(stmt_id);
    let response = client.billing_statements().mark_uncollectible(&id).await?;

    assert_eq!(response.id, id);
    Ok(())
}
