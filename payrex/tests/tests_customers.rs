mod support;

use payrex::Client;
use payrex::Error as PayrexError;
use payrex::resources::customers::UpdateCustomer;
use payrex::types::{CustomerId, Metadata};
use serde_json::Value;
use wiremock::{
    Mock, MockBuilder, MockServer, ResponseTemplate,
    matchers::{basic_auth, body_string_contains, method, path},
};

use crate::support::{Result, TEST_API_KEY, create_json_fixture, mock_config};

const CUSTOMER_FIXTURE: &str = include_str!("fixtures/customer.json");

fn mock_customer_builder(method_str: &str, path_param: Option<&str>) -> MockBuilder {
    Mock::given(method(method_str))
        .and(path(format!("/customers{}", path_param.unwrap_or(""))))
        .and(basic_auth(TEST_API_KEY, ""))
}

#[tokio::test]
async fn test_retrieve_customer_ok() -> Result<()> {
    let mock_server = MockServer::start().await;
    let json_body = create_json_fixture(CUSTOMER_FIXTURE);
    let customer_id = "cus_8Te4pwkR5ePwG2UVsY2NTJyVDXYaVQLX";

    mock_customer_builder("GET", Some(format!("/{customer_id}").as_ref()))
        .respond_with(ResponseTemplate::new(200).set_body_json(json_body))
        .mount(&mock_server)
        .await;

    let config = mock_config(mock_server.uri())?;
    let client = Client::with_config(config)?;
    let id = CustomerId::new(customer_id);
    let response = client.customers().retrieve(&id).await?;

    assert_eq!(response.id, id);
    assert_eq!(response.email.as_deref(), Some("testdev@gmail.com"));
    assert_eq!(response.name.as_deref(), Some("Test dev"));
    assert_eq!(
        response.billing_statement_prefix.as_deref(),
        Some("SAB9EQZG")
    );
    assert_eq!(
        response.next_billing_statement_sequence_number.as_deref(),
        Some("7")
    );
    Ok(())
}

#[tokio::test]
async fn test_update_customer_ok() -> Result<()> {
    let mock_server = MockServer::start().await;
    let mut json_body = create_json_fixture(CUSTOMER_FIXTURE);
    let customer_id = "cus_8Te4pwkR5ePwG2UVsY2NTJyVDXYaVQLX";

    json_body["email"] = Value::String("new@example.com".to_string());

    mock_customer_builder("PUT", Some(format!("/{customer_id}").as_ref()))
        .and(body_string_contains("email=new%40example.com"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json_body))
        .mount(&mock_server)
        .await;

    let config = mock_config(mock_server.uri())?;
    let client = Client::with_config(config)?;
    let id = CustomerId::new(customer_id);
    let params = UpdateCustomer::new().email("new@example.com");
    let response = client.customers().update(&id, params).await?;

    assert_eq!(response.email.as_deref(), Some("new@example.com"));
    Ok(())
}
