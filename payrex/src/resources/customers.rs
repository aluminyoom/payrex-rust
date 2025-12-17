//! Customers API
//!
//! Customers represent your business's customers and allow you to track
//! multiple payments and billing information.

use crate::{
    Result,
    http::HttpClient,
    types::{Currency, CustomerId, List, ListParams, Metadata, Timestamp},
};
use payrex_derive::{Payrex, payrex_attr};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Customers API
#[derive(Clone)]
pub struct Customers {
    http: Arc<HttpClient>,
}

impl Customers {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Creates a customer resource.
    ///
    /// Endpoint: `POST /customers`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/customers/create)
    pub async fn create(&self, params: CreateCustomer) -> Result<Customer> {
        self.http.post("/customers", &params).await
    }

    /// Retrieves a customer resource.
    ///
    /// Endpoint: `GET /customers/:id`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/customers/retrieve)
    pub async fn retrieve(&self, id: &CustomerId) -> Result<Customer> {
        self.http.get(&format!("/customers/{}", id.as_str())).await
    }

    /// Updates a customer resource.
    ///
    /// Endpoint: `PUT /customers/:id`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/customers/update)
    pub async fn update(&self, id: &CustomerId, params: UpdateCustomer) -> Result<Customer> {
        self.http
            .patch(&format!("/customers/{}", id.as_str()), &params)
            .await
    }

    /// Deletes a customer resource.
    ///
    /// Deleted customers can still be retrieved through the retrieve customer endpoint to track
    /// their history.
    ///
    /// Endpoint: `DELETE /customers/:id`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/customers/delete)
    pub async fn delete(&self, id: &CustomerId) -> Result<()> {
        self.http
            .delete(&format!("/customers/{}", id.as_str()))
            .await
    }

    /// List customer resources.
    ///
    /// Endpoint: `GET /customers`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/customers/list)
    pub async fn list(&self, params: Option<CustomerListParams>) -> Result<List<Customer>> {
        self.http.get_with_params("/customers", &params).await
    }
}

/// A Customer resource represents the customer of your business. A customer could be a person or a
/// company. Use this resource to track payments that belong to the same customer.
#[payrex_attr(livemode, metadata, timestamp, optional, currency = true)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Customer {
    /// Unique identifier for the resource. The prefix is `cus_`.
    pub id: CustomerId,

    /// The customer's prefix used to generate unique billing statement numbers.
    ///
    /// To learn more about billing statements, you can check this
    /// [guide](https://docs.payrexhq.com/docs/guide/finance_automation/billing_statements/overview).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_statement_prefix: Option<String>,

    /// The customer's e-mail address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// The customer's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The suffix of the customer's next billing statement number, e.g. 0001. PayRex manages this
    /// sequence number when you associate a customer with a billing statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_billing_statement_sequence_number: Option<String>,
}

/// Query parameters when creating a customer.
///
/// [Reference](https://docs.payrexhq.com/docs/api/customers/create#parameters)
#[payrex_attr(metadata, currency = false)]
#[derive(Debug, Clone, Default, Serialize, Deserialize, Payrex)]
pub struct CreateCustomer {
    /// The customer's e-mail address.
    pub email: String,

    /// The customer's name.
    pub name: String,

    /// The customer's prefix used to generate unique billing statement numbers. Allows 3-15
    /// uppercase letters or numbers.
    ///
    /// To learn more about billing statements, you can check this
    /// [guide](https://docs.payrexhq.com/docs/guide/finance_automation/billing_statements/overview).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[payrex(
        description = "Sets the billing statement prefix in the query params when creating a customer."
    )]
    pub billing_statement_prefix: Option<String>,

    /// The sequence number used as a suffix when creating the customer's next billing statement
    /// number. Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[payrex(
        description = "Sets the next billing statement sequence number in the query params when creating a customer."
    )]
    pub next_billing_statement_sequence_number: Option<String>,
}

/// Query parameters when updating a customer.
///
/// [Reference](https://docs.payrexhq.com/docs/api/customers/update#parameters)
#[payrex_attr(metadata, currency = true)]
#[derive(Debug, Clone, Default, Serialize, Deserialize, Payrex)]
pub struct UpdateCustomer {
    /// The customer's prefix used to generate unique billing statement numbers. Allows 3-15
    /// uppercase letters or numbers.
    ///
    /// To learn more about billing statements, you can check this
    /// [guide](https://docs.payrexhq.com/docs/guide/finance_automation/billing_statements/overview).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[payrex(
        description = "Sets the billing statement prefix in the query params when updating a customer."
    )]
    pub billing_statement_prefix: Option<String>,

    /// The sequence number used as a suffix when creating the customer's next billing statement
    /// number. Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[payrex(
        description = "Sets the next billing statement sequence number in the query params when updating a customer."
    )]
    pub next_billing_statement_sequence_number: Option<String>,

    /// The customer's e-mail address.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[payrex(description = "Sets the email in query params when updating a customer.")]
    pub email: Option<String>,

    /// The customer's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[payrex(description = "Sets the name in query params when updating a customer.")]
    pub name: Option<String>,
}

/// Query parameters when listing customers.
///
/// [Reference](https://docs.payrexhq.com/docs/api/customers/list#parameters)
#[payrex_attr(metadata)]
#[derive(Debug, Clone, Default, Serialize, Deserialize, Payrex)]
pub struct CustomerListParams {
    /// Baseline pagination fields such as `limit`, `before`, and `after`.
    #[serde(flatten)]
    pub list_params: ListParams,

    /// The customer's e-mail address.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[payrex(description = "Sets the email in query params when listing customers.")]
    pub email: Option<String>,

    /// The customer's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[payrex(description = "Sets the name in query params when listing customers.")]
    pub name: Option<String>,
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
            "test@example.com".to_string(),
            "Test User".to_string(),
            Currency::PHP,
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
