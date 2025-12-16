//! Customers API
//!
//! Customers represent your business's customers and allow you to track
//! multiple payments and billing information.

use crate::{
    Result,
    http::HttpClient,
    types::{Currency, CustomerId, List, ListParams, Metadata, Timestamp},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct Customers {
    http: Arc<HttpClient>,
}

impl Customers {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub async fn create(&self, params: CreateCustomer) -> Result<Customer> {
        self.http.post("/customers", &params).await
    }

    pub async fn retrieve(&self, id: &CustomerId) -> Result<Customer> {
        self.http.get(&format!("/customers/{}", id.as_str())).await
    }

    pub async fn update(&self, id: &CustomerId, params: UpdateCustomer) -> Result<Customer> {
        self.http
            .patch(&format!("/customers/{}", id.as_str()), &params)
            .await
    }

    pub async fn delete(&self, id: &CustomerId) -> Result<()> {
        self.http
            .delete(&format!("/customers/{}", id.as_str()))
            .await
    }

    pub async fn list(&self, params: Option<CustomerListParams>) -> Result<List<Customer>> {
        self.http.get_with_params("/customers", &params).await
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Customer {
    pub id: CustomerId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_statement_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_billing_statement_sequence_number: Option<String>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptionalCustomer {
    pub id: CustomerId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_statement_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub livemode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_billing_statement_sequence_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateCustomer {
    pub currency: Currency,
    pub email: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_statement_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_billing_statement_sequence_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateCustomer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_statement_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_billing_statement_sequence_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomerListParams {
    #[serde(flatten)]
    pub list_params: ListParams,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl CreateCustomer {
    #[must_use]
    pub fn new(currency: Currency, email: String, name: String) -> Self {
        Self {
            currency,
            email,
            name,
            ..Default::default()
        }
    }

    pub fn billing_statement_prefix(mut self, billing_statement_prefix: impl Into<String>) -> Self {
        self.billing_statement_prefix = Some(billing_statement_prefix.into());
        self
    }

    pub fn next_billing_statement_sequence_number(
        mut self,
        sequence_number: impl Into<String>,
    ) -> Self {
        self.next_billing_statement_sequence_number = Some(sequence_number.into());
        self
    }

    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

// TODO: maybe consider `derive_builder` crate
impl UpdateCustomer {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn currency(mut self, currency: Currency) -> Self {
        self.currency = Some(currency);
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn billing_statement_prefix(mut self, billing_statement_prefix: impl Into<String>) -> Self {
        self.billing_statement_prefix = Some(billing_statement_prefix.into());
        self
    }

    pub fn next_billing_statement_sequence_number(
        mut self,
        sequence_number: impl Into<String>,
    ) -> Self {
        self.next_billing_statement_sequence_number = Some(sequence_number.into());
        self
    }

    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

impl CustomerListParams {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Currency, CustomerId, ListParams, Metadata, Timestamp};
    use serde_json;

    #[test]
    fn test_create_customer_builder() {
        let mut metadata = Metadata::new();
        metadata.insert("order_id".to_string(), "12345".to_string());
        let params = CreateCustomer::new(
            Currency::PHP,
            "test@example.com".to_string(),
            "Test User".to_string(),
        )
        .billing_statement_prefix("PKYG9MA2")
        .next_billing_statement_sequence_number("002")
        .metadata(metadata.clone());
        assert_eq!(params.currency, Currency::PHP);
        assert_eq!(params.email, "test@example.com".to_string());
        assert_eq!(params.name, "Test User".to_string());
        assert_eq!(
            params.billing_statement_prefix,
            Some("PKYG9MA2".to_string())
        );
        assert_eq!(
            params.next_billing_statement_sequence_number,
            Some("002".to_string())
        );
        assert_eq!(params.metadata, Some(metadata));
    }

    #[test]
    fn test_update_customer_builder() {
        let mut metadata = Metadata::new();
        metadata.insert("key".to_string(), "value".to_string());
        let params = UpdateCustomer::new()
            .currency(Currency::PHP)
            .email("user@example.com")
            .name("User")
            .billing_statement_prefix("BS")
            .next_billing_statement_sequence_number("003")
            .metadata(metadata.clone());
        assert_eq!(params.currency, Some(Currency::PHP));
        assert_eq!(params.email, Some("user@example.com".to_string()));
        assert_eq!(params.name, Some("User".to_string()));
        assert_eq!(params.billing_statement_prefix, Some("BS".to_string()));
        assert_eq!(
            params.next_billing_statement_sequence_number,
            Some("003".to_string())
        );
        assert_eq!(params.metadata, Some(metadata));
    }

    #[test]
    fn test_customer_list_params_builder() {
        let mut metadata = Metadata::new();
        metadata.insert("key", "value");
        let mut params = CustomerListParams::new()
            .name("Name")
            .email("user@example.com")
            .metadata(metadata.clone());
        params.list_params = ListParams::new()
            .limit(20)
            .after("cus_abc")
            .before("cus_def");
        assert_eq!(params.list_params.limit, Some(20));
        assert_eq!(params.list_params.after.as_deref(), Some("cus_abc"));
        assert_eq!(params.list_params.before.as_deref(), Some("cus_def"));
        assert_eq!(params.name, Some("Name".to_string()));
        assert_eq!(params.email, Some("user@example.com".to_string()));
        assert_eq!(params.metadata.unwrap().get("key"), Some("value"));
    }

    #[test]
    fn test_customer_serialization() {
        let mut metadata = Metadata::new();
        metadata.insert("order_id", "12345");
        let customer = Customer {
            id: CustomerId::new("cus_123456"),
            billing_statement_prefix: Some("PREF".to_string()),
            currency: Some(Currency::PHP),
            email: Some("test@example.com".to_string()),
            livemode: false,
            name: Some("Test User".to_string()),
            metadata: Some(metadata.clone()),
            next_billing_statement_sequence_number: Some("004".to_string()),
            created_at: Timestamp::from_unix(1_609_459_200),
            updated_at: Timestamp::from_unix(1_609_459_300),
        };
        let json = serde_json::to_value(&customer).unwrap();
        assert_eq!(json["id"], "cus_123456");
        assert_eq!(json["billing_statement_prefix"], "PREF");
        assert_eq!(json["currency"], "PHP");
        assert_eq!(json["email"], "test@example.com");
        assert_eq!(json["livemode"], false);
        assert_eq!(json["name"], "Test User");
        assert_eq!(json["metadata"]["order_id"], "12345");
        assert_eq!(json["next_billing_statement_sequence_number"], "004");
        assert_eq!(json["created_at"], 1_609_459_200);
        assert_eq!(json["updated_at"], 1_609_459_300);
    }

    #[test]
    fn test_customer_list_params_serialization() {
        let json_in = r#"
        {
            "limit": 10,
            "after": "cus_123",
            "email": "user@example.com",
            "name": "User Name",
            "metadata": {"foo": "bar"}
        }"#;
        let params: CustomerListParams = serde_json::from_str(json_in).unwrap();
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["limit"], 10);
        assert_eq!(json["after"], "cus_123");
        assert_eq!(json["email"], "user@example.com");
        assert_eq!(json["name"], "User Name");
        assert_eq!(json["metadata"]["foo"], "bar");
    }
}
