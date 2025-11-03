//! Billing Statement Line Items API
//!
//! Billing Statement Line Items allows you to create, update, and delete statement line items.

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{
    Result,
    http::HttpClient,
    types::{BillingStatementId, BillingStatementLineItemId, Timestamp},
};

#[derive(Clone)]
pub struct BillingStatementLineItems {
    http: Arc<HttpClient>,
}

impl BillingStatementLineItems {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub async fn create(
        &self,
        params: CreateBillingStatementLineItem,
    ) -> Result<BillingStatementLineItem> {
        self.http
            .post("/billing_statement_line_items", &params)
            .await
    }

    pub async fn update(
        &self,
        id: BillingStatementLineItemId,
        params: UpdateBillingStatementLineItem,
    ) -> Result<BillingStatementLineItem> {
        self.http
            .put(
                &format!("/billing_statement_line_items/{}", id.as_str()),
                &params,
            )
            .await
    }

    pub async fn delete(&self, id: &BillingStatementLineItemId) -> Result<()> {
        self.http
            .delete(&format!("/billing_statement_line_items/{}", id.as_str()))
            .await
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BillingStatementLineItem {
    pub id: BillingStatementLineItemId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub unit_price: u64,
    pub quantity: u64,
    pub billing_statement_id: BillingStatementId,
    pub livemode: bool,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateBillingStatementLineItem {
    pub billing_statement_id: BillingStatementId,
    pub description: String,
    pub unit_price: u64,
    pub quantity: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UpdateBillingStatementLineItem {
    pub description: Option<String>,
    pub unit_price: Option<u64>,
    pub quantity: Option<u64>,
}

impl CreateBillingStatementLineItem {
    #[must_use]
    pub fn new(
        billing_statement_id: BillingStatementId,
        description: impl Into<String>,
        unit_price: u64,
        quantity: u64,
    ) -> Self {
        Self {
            billing_statement_id,
            description: description.into(),
            unit_price,
            quantity,
        }
    }
}

impl UpdateBillingStatementLineItem {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn unit_price(mut self, price: u64) -> Self {
        self.unit_price = Some(price);
        self
    }

    pub fn quantity(mut self, quantity: u64) -> Self {
        self.quantity = Some(quantity);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{BillingStatementId, BillingStatementLineItemId, Timestamp};
    use serde_json;

    #[test]
    fn test_create_billing_statement_line_item_builder() {
        let params = CreateBillingStatementLineItem::new(
            BillingStatementId::new("bstm_1"),
            "Item A",
            1500,
            3,
        );
        assert_eq!(params.billing_statement_id.as_str(), "bstm_1");
        assert_eq!(params.description, "Item A".to_string());
        assert_eq!(params.unit_price, 1500);
        assert_eq!(params.quantity, 3);
    }

    #[test]
    fn test_update_billing_statement_line_item_builder() {
        let params = UpdateBillingStatementLineItem::new()
            .description("Updated item")
            .unit_price(2000)
            .quantity(5);
        assert_eq!(params.description, Some("Updated item".to_string()));
        assert_eq!(params.unit_price, Some(2000));
        assert_eq!(params.quantity, Some(5));
    }

    #[test]
    fn test_billing_statement_line_item_serialization() {
        let item = BillingStatementLineItem {
            id: BillingStatementLineItemId::new("bstm_li_1"),
            description: Some("Test item".to_string()),
            unit_price: 1200,
            quantity: 2,
            billing_statement_id: BillingStatementId::new("bstm_1"),
            livemode: false,
            created_at: Timestamp::from_unix(1_621_000_000),
            updated_at: Timestamp::from_unix(1_621_000_100),
        };
        let json = serde_json::to_value(&item).unwrap();
        assert_eq!(json["id"], "bstm_li_1");
        assert_eq!(json["description"], "Test item");
        assert_eq!(json["unit_price"], 1200);
        assert_eq!(json["quantity"], 2);
        assert_eq!(json["billing_statement_id"], "bstm_1");
        assert_eq!(json["livemode"], false);
        assert_eq!(json["created_at"], 1_621_000_000);
        assert_eq!(json["updated_at"], 1_621_000_100);
    }

    #[test]
    fn test_update_billing_statement_line_item_serialization() {
        let params = UpdateBillingStatementLineItem::new()
            .description("Example description")
            .unit_price(500)
            .quantity(1);
        let serialized = serde_json::to_string(&params).unwrap();
        assert_eq!(
            serialized,
            r#"{"description":"Example description","unit_price":500,"quantity":1}"#
        );
    }
}
