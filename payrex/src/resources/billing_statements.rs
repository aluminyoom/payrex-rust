//! Billing Statements API
//!
//! Billing Statements allow you to create and send invoices to customers.

use crate::resources::billing_statement_line_items::BillingStatementLineItem;
use crate::resources::payment_intents::OptionalPaymentIntent;
use crate::{
    Result,
    http::HttpClient,
    resources::customers::OptionalCustomer,
    types::{
        BillingStatementId, Currency, CustomerId, List, ListParams, Metadata, PaymentMethod,
        Timestamp,
    },
};
use payrex_derive::{Payrex, payrex_attr};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
/// Billing statements are one-time payment links that contain customer information, the due date,
/// and an itemized list of your business's products or services.
pub struct BillingStatements {
    http: Arc<HttpClient>,
}

impl BillingStatements {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Creates a billing statement resource.
    ///
    /// Endpoint: `POST /billing_statements`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/billing_statements/create)
    pub async fn create(&self, params: CreateBillingStatement) -> Result<BillingStatement> {
        self.http.post("/billing_statements", &params).await
    }

    /// Retrieves a billing statement resource.
    ///
    /// Endpoint: `GET /billing_statements/:id`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/billing_statements/retrieve)
    pub async fn retrieve(&self, id: &BillingStatementId) -> Result<BillingStatement> {
        self.http
            .get(&format!("/billing_statements/{}", id.as_str()))
            .await
    }

    /// Updates a billing statement resource.
    ///
    /// Endpoint: `PUT /billing_statements/:id`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/billing_statements/update)
    pub async fn update(
        &self,
        id: &BillingStatementId,
        params: UpdateBillingStatement,
    ) -> Result<BillingStatement> {
        self.http
            .put(&format!("/billing_statements/{}", id.as_str()), &params)
            .await
    }

    /// Deletes a billing statement resource.
    ///
    /// Endpoint: `DELETE /billing_statements/:id`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/billing_statements/delete)
    pub async fn delete(&self, id: &BillingStatementId) -> Result<()> {
        self.http
            .delete(&format!("/billing_statements/{}", id.as_str()))
            .await
    }

    /// List billing statement resources.
    ///
    /// Endpoint: `GET /billing_statements`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/billing_statements/list)
    pub async fn list(&self, params: Option<ListParams>) -> Result<List<BillingStatement>> {
        self.http
            .get_with_params("/billing_statements", &params)
            .await
    }

    /// Finalizes a billing statement resource.
    ///
    /// Endpoint: `POST /billing_statements/:id/finalize`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/billing_statements/finalize)
    pub async fn finalize(&self, id: &BillingStatementId) -> Result<BillingStatement> {
        self.http
            .post(
                &format!("/billing_statements/{}/finalize", id.as_str()),
                &(),
            )
            .await
    }

    /// Send a billing statement via e-mail.
    ///
    /// Endpoint: `POST /billing_statements/:id/send`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/billing_statements/send)
    pub async fn send(&self, id: &BillingStatementId) -> Result<BillingStatement> {
        self.http
            .post(&format!("/billing_statements/{}/send", id.as_str()), &())
            .await
    }

    /// Voids a billing statement resource.
    ///
    /// Endpoint: `POST /billing_statements/:id/void`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/billing_statements/void)
    pub async fn void(&self, id: &BillingStatementId) -> Result<BillingStatement> {
        self.http
            .post(&format!("/billing_statements/{}/void", id.as_str()), &())
            .await
    }

    /// Mark uncollectible a billing statement resource.
    ///
    /// Endpoint: `POST /billing_statements/:id/mark_uncollectible`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/billing_statements/mark_uncollectible)
    pub async fn mark_uncollectible(&self, id: &BillingStatementId) -> Result<BillingStatement> {
        self.http
            .post(
                &format!("/billing_statements/{}/mark_uncollectible", id.as_str()),
                &(),
            )
            .await
    }
}

/// Billing Statement Resource.
///
/// [Learn more about it here](https://docs.payrexhq.com/docs/api/billing_statements)
#[payrex_attr(
    timestamp,
    livemode,
    metadata,
    currency = false,
    amount = false,
    description = "billing_statements"
)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BillingStatement {
    /// The ID of a customer resource. To learn more about the customer resource, you can refer
    /// [here](https://docs.payrexhq.com/docs/api/customers).
    pub id: BillingStatementId,

    /// Defines if the billing information fields will always show or managed by PayRex. Default value
    /// is `always`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details_collection: Option<String>,

    /// The ID of a customer resource. To learn more about the customer resource, you can refer
    /// [here](https://docs.payrexhq.com/docs/api/customers).
    pub customer_id: CustomerId,

    /// The time when the billing statement is expected to be paid. If the `due_at` is already past,
    /// your customer can still pay the billing statement if the status is open.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_at: Option<Timestamp>,

    /// The time when a billing statement was finalized.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finalized_at: Option<Timestamp>,

    /// The name of the merchant where the billing statement belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_statement_merchant_name: Option<String>,

    /// The number associated with a billing statement.
    // TODO: Consider using u64 instead
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_statement_number: Option<String>,

    /// The URL that your customer will access to pay the billing statement.
    ///
    /// This is only visible if the billing statement's status [`BillingStatementStatus`] is `open`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_statement_url: Option<String>,

    /// This attribute holds the billing statement's list of line items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<BillingStatementLineItem>>,

    /// The [PaymentIntent](https://docs.payrexhq.com/docs/api/payment_intents) resource created
    /// for the [`BillingStatement`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<OptionalPaymentIntent>,

    /// The setup for future usage of this billing statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<String>,

    /// This attribute holds the statement descriptor for the billing statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// The latest status of the BillingStatement. Possible values are open, draft, paid, void or uncollectible.
    pub status: BillingStatementStatus,

    /// Set of key-value pairs that can modify the behavior of the payment processing for the
    /// billing statement.
    pub payment_settings: PaymentSettings,

    /// A customer resource that is associated with the billing statement (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<OptionalCustomer>,
}

/// Payment Settings for a billing statement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaymentSettings {
    /// The list of payment methods allowed to be processed by the payment intent of the billing
    /// statement.
    pub payment_methods: Vec<PaymentMethod>,
}

/// The latest status of the [`BillingStatement`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BillingStatementStatus {
    /// The latest status is draft.
    Draft,

    /// The latest status is open.
    Open,

    /// The latest status is paid.
    Paid,

    /// The latest status is uncollectible.
    Void,

    /// The latest status is uncollectible.
    Uncollectible,
}

/// Query parameters when creating a billing statement.
///
/// [Reference](https://docs.payrexhq.com/docs/api/billing_statements/create#parameters)
#[payrex_attr(metadata, currency = false, description = "billing_statements")]
#[derive(Debug, Default, Clone, Serialize, Deserialize, Payrex)]
pub struct CreateBillingStatement {
    /// The ID of a customer resource. To learn more about the customer resource, you can refer
    /// [here](https://docs.payrexhq.com/docs/api/customers).
    pub customer_id: CustomerId,

    /// Set of key-value pairs that can modify the behavior of the payment processing for the
    /// billing statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[payrex(description = "Sets the payment settings when creating a billing statement.")]
    pub payment_settings: Option<PaymentSettings>,

    /// Defines if the billing information fields will always show or managed by PayRex. Default value is `always`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[payrex(
        description = "Sets the billing details collection when creating a billing statement."
    )]
    pub billing_details_collection: Option<String>,
}

/// Query parameters when updating a billing statement.
///
/// [Reference](https://docs.payrexhq.com/docs/api/billing_statements/update#parameters)
#[payrex_attr(metadata, description = "billing_statements")]
#[derive(Debug, Default, Clone, Serialize, Deserialize, Payrex)]
pub struct UpdateBillingStatement {
    /// The ID of a customer resource. To learn more about the customer resource, you can refer
    /// [here](https://docs.payrexhq.com/docs/api/customers).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[payrex(description = "Sets the customer ID before updating a billing statement.")]
    pub customer_id: Option<CustomerId>,

    /// Set of key-value pairs that can modify the behavior of the payment processing for the
    /// billing statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[payrex(description = "Sets the payment settings before updating a billing statement.")]
    pub payment_settings: Option<PaymentSettings>,

    /// Defines if the billing information fields will always show or managed by PayRex.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[payrex(
        description = "Sets the billing details collection before updating a billing statement."
    )]
    pub billing_details_collection: Option<String>,

    /// The time when the billing statement is expected to be paid. If the due_at is already past, your customer can still pay the billing statement if the status is open.
    ///
    /// Measured in seconds since the Unix epoch.
    #[payrex(description = "Sets the deadline for the billing statement at a specified date.")]
    pub due_at: Option<Timestamp>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::{BillingStatementStatus, PaymentSettings};
    use crate::types::BillingStatementLineItemId;
    use crate::types::{
        BillingStatementId, Currency, CustomerId, Metadata, PaymentMethod, Timestamp,
    };
    use serde_json;

    #[test]
    fn test_billing_statement_status_serialization() {
        assert_eq!(
            serde_json::to_string(&BillingStatementStatus::Draft).unwrap(),
            "\"draft\""
        );
        assert_eq!(
            serde_json::to_string(&BillingStatementStatus::Open).unwrap(),
            "\"open\""
        );
        assert_eq!(
            serde_json::to_string(&BillingStatementStatus::Paid).unwrap(),
            "\"paid\""
        );
        assert_eq!(
            serde_json::to_string(&BillingStatementStatus::Void).unwrap(),
            "\"void\""
        );
        assert_eq!(
            serde_json::to_string(&BillingStatementStatus::Uncollectible).unwrap(),
            "\"uncollectible\""
        );
    }

    #[test]
    fn test_payment_settings_serialization() {
        let settings = PaymentSettings {
            payment_methods: vec![PaymentMethod::Card, PaymentMethod::GCash],
        };

        let json = serde_json::to_value(&settings).unwrap();
        let methods = json["payment_methods"].as_array().unwrap();
        assert_eq!(methods[0].as_str().unwrap(), "card");
        assert_eq!(methods[1].as_str().unwrap(), "gcash");
    }

    #[test]
    fn test_create_billing_statement_builder() {
        let mut metadata = Metadata::new();
        metadata.insert("k", "v");

        let settings = PaymentSettings {
            payment_methods: vec![PaymentMethod::QRPh],
        };

        let params = CreateBillingStatement::new(CustomerId::new("cus_001"), Currency::PHP)
            .payment_settings(settings.clone())
            .billing_details_collection("always")
            .description("desc")
            .metadata(metadata.clone());

        assert_eq!(params.customer_id.as_str(), "cus_001");
        assert_eq!(params.currency, Currency::PHP);
        assert_eq!(params.payment_settings, Some(settings));
        assert_eq!(params.billing_details_collection.as_deref(), Some("always"));
        assert_eq!(params.description.as_deref(), Some("desc"));
        assert_eq!(params.metadata.unwrap().get("k"), Some("v"));
    }

    #[test]
    fn test_update_billing_statement_serialization() {
        let mut metadata = Metadata::new();
        metadata.insert("x", "y");

        let settings = PaymentSettings {
            payment_methods: vec![PaymentMethod::Maya],
        };

        let params = UpdateBillingStatement::new()
            .customer_id(CustomerId::new("cus_002"))
            .payment_settings(settings.clone())
            .billing_details_collection("always")
            .description("upd")
            .metadata(metadata.clone());

        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["customer_id"], "cus_002");
        assert_eq!(json["billing_details_collection"], "always");
        assert_eq!(json["payment_settings"]["payment_methods"][0], "maya");
        assert_eq!(json["description"], "upd");
        assert_eq!(json["metadata"]["x"], "y");
    }

    #[test]
    fn test_billing_statement_serialization() {
        let mut metadata = Metadata::new();
        metadata.insert("foo", "bar");

        let settings = PaymentSettings {
            payment_methods: vec![PaymentMethod::QRPh],
        };

        let item = BillingStatementLineItem {
            id: BillingStatementLineItemId::new("bstm_li_1"),
            description: Some("Test item".to_string()),
            unit_price: 1500,
            quantity: 2,
            billing_statement_id: BillingStatementId::new("bstm_123"),
            livemode: false,
            created_at: Timestamp::from_unix(1_620_003_000),
            updated_at: Timestamp::from_unix(1_620_003_000),
        };
        let stmt = BillingStatement {
            id: BillingStatementId::new("bstm_123"),
            amount: 2000,
            billing_details_collection: Some("mandatory".to_string()),
            currency: Currency::PHP,
            customer_id: CustomerId::new("cus_999"),
            description: Some("Test invoice".to_string()),
            due_at: Some(Timestamp::from_unix(1_620_002_000)),
            finalized_at: None,
            billing_statement_merchant_name: Some("Shop".to_string()),
            billing_statement_number: Some("BS100".to_string()),
            billing_statement_url: Some("http://example.com".to_string()),
            line_items: Some(vec![item.clone()]),
            livemode: false,
            metadata: Some(metadata.clone()),
            payment_intent: None,
            setup_future_usage: Some("on_session".to_string()),
            statement_descriptor: Some("DESC".to_string()),
            status: BillingStatementStatus::Open,
            payment_settings: settings.clone(),
            customer: None,
            created_at: Timestamp::from_unix(1_620_000_000),
            updated_at: Timestamp::from_unix(1_620_001_000),
        };

        let json = serde_json::to_value(&stmt).unwrap();
        assert_eq!(json["id"], "bstm_123");
        assert_eq!(json["amount"], 2000);
        assert_eq!(json["billing_details_collection"], "mandatory");
        assert_eq!(json["currency"], "PHP");
        assert_eq!(json["customer_id"], "cus_999");
        assert_eq!(json["description"], "Test invoice");
        assert_eq!(json["due_at"], 1_620_002_000);
        assert_eq!(json["billing_statement_number"], "BS100");
        assert_eq!(json["billing_statement_url"], "http://example.com");

        let items = json["line_items"].as_array().unwrap();
        assert_eq!(items[0]["id"], "bstm_li_1");
        assert_eq!(items[0]["description"], "Test item");
        assert_eq!(items[0]["unit_price"], 1500);
        assert_eq!(items[0]["quantity"], 2);
        assert_eq!(items[0]["billing_statement_id"], "bstm_123");
        assert_eq!(items[0]["livemode"], false);
        assert_eq!(items[0]["created_at"], 1_620_003_000);
        assert_eq!(items[0]["updated_at"], 1_620_003_000);
        assert_eq!(json["livemode"], false);
        assert_eq!(json["metadata"]["foo"], "bar");
        assert_eq!(json["setup_future_usage"], "on_session");
        assert_eq!(json["statement_descriptor"], "DESC");
        assert_eq!(json["status"], "open");
        let methods = json["payment_settings"]["payment_methods"]
            .as_array()
            .unwrap();
        assert_eq!(methods[0].as_str().unwrap(), "qrph");
        assert_eq!(json["created_at"], 1_620_000_000);
        assert_eq!(json["updated_at"], 1_620_001_000);
    }
}
