mod support;

use payrex::Client;
use payrex::resources::payouts::PayoutTransactionType;
use payrex::types::{PayoutId, PayoutTransactionId};
use serde_json::Value;
use wiremock::{
    Mock, MockBuilder, MockServer, ResponseTemplate,
    matchers::{basic_auth, method, path},
};

use crate::support::{Result, TEST_API_KEY, create_json_fixture, mock_config};

const PAYOUT_FIXTURE: &str = include_str!("fixtures/payout.json");

fn mock_payout_builder(method_str: &str, path_param: Option<&str>) -> MockBuilder {
    Mock::given(method(method_str))
        .and(path(format!(
            "/payouts{}/transactions",
            path_param.unwrap_or("")
        )))
        .and(basic_auth(TEST_API_KEY, ""))
}

#[tokio::test]
async fn test_list_payout_transactions_ok() -> Result<()> {
    let mock_server = MockServer::start().await;
    let json_body = create_json_fixture(PAYOUT_FIXTURE);
    let payout_id = "po_eVG7pzEk7hLWFUpA6nj9pj4BQTk68kXb";

    mock_payout_builder("GET", Some(format!("/{payout_id}").as_ref()))
        .respond_with(ResponseTemplate::new(200).set_body_json(json_body.clone()))
        .mount(&mock_server)
        .await;

    let config = mock_config(mock_server.uri())?;
    let client = Client::with_config(config)?;
    let id = PayoutId::new(payout_id);
    let list = client.payouts().list_transactions(&id, None).await?;

    let txn = &list.data[0];

    assert_eq!(list.data.len(), 1);
    assert_eq!(
        txn.id,
        PayoutTransactionId::new("po_txn_i6P2DdBm6vdKYeAhnCuaWj9Hb5iBxQEX")
    );
    assert_eq!(txn.net_amount, 51324);
    assert_eq!(txn.transaction_type, PayoutTransactionType::Payment);
    Ok(())
}
