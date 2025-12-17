//! Billing Statement Line Items API
//!
//! Billing Statement Line Items allows you to create, update, and delete statement line items.

use std::sync::Arc;

use payrex_derive::{Payrex, payrex_attr};
use serde::{Deserialize, Serialize};

use crate::{
    Result,
    http::HttpClient,
    types::{BillingStatementId, BillingStatementLineItemId, Timestamp},
};

/// Billing Statement Lines API
#[derive(Clone)]
pub struct BillingStatementLineItems {
    http: Arc<HttpClient>,
}

impl BillingStatementLineItems {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Creates a billing statement line item resource.
    ///
    /// Endpoint: `POST /billing_statement_line_items`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/billing_statement_line_items/create)
    pub async fn create(
        &self,
        params: CreateBillingStatementLineItem,
    ) -> Result<BillingStatementLineItem> {
        self.http
            .post("/billing_statement_line_items", &params)
            .await
    }

    /// Updates a billing statement line item resource.
    ///
    /// Endpoint: `PUT /billing_statement_line_items/:id`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/billing_statement_line_items/update)
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

    /// Deletes a billing statement line item resource.
    ///
    /// Endpoint: `DELETE /billing_statement_line_items/:id`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/billing_statement_line_items/delete)
    pub async fn delete(&self, id: &BillingStatementLineItemId) -> Result<()> {
        self.http
            .delete(&format!("/billing_statement_line_items/{}", id.as_str()))
            .await
    }
}

/// The billing statement line item is a line item of a billing statement that pertains to a
/// business's products or services.
#[payrex_attr(livemode, timestamp, description = "billing_statement_line_items")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BillingStatementLineItem {
    /// Unique identifier for the resource. The prefix is `bstm_li_`.
    pub id: BillingStatementLineItemId,

    /// The amount of the line item in a single unit.
    pub unit_price: u64,

    /// The quantity of the line item. The quantity will be multiplied by the line_item.amount to
    /// compute the final amount of the billing statement.
    ///
    /// This is a positive integer in the smallest currency unit, cents. If the line item's unit
    /// price is ₱ 120.50, the value should be 12050.
    pub quantity: u64,

    /// The ID of the billing statement where the line item is associated.
    pub billing_statement_id: BillingStatementId,
}

/// Query parameters when creating a billing statement line item.
///
/// [Reference](https://docs.payrexhq.com/docs/api/billing_statement_line_items/create#parameters)
#[payrex_attr(description = "billing_statement_line_items")]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Payrex)]
pub struct CreateBillingStatementLineItem {
    /// The ID of a billing statement resource. To learn more about the billing statement resource,
    /// refer [here](https://docs.payrexhq.com/docs/api/billing_statements).
    pub billing_statement_id: BillingStatementId,

    /// The amount of the line item in a single unit.
    ///
    /// This is a positive integer in the smallest currency unit, cents. If the line item should be
    /// ₱ 120.50, the amount should be 12050.
    pub unit_price: u64,

    /// The quantity of the line item. The quantity will be multiplied by the line_item.amount to
    /// compute the final amount of the billing statement.
    pub quantity: u64,
}

/// Query parameters when updating a billing statement line item.
///
/// [Reference](https://docs.payrexhq.com/docs/api/billing_statement_line_items/update#parameters)
#[payrex_attr(description = "billing_statement_line_items")]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Payrex)]
pub struct UpdateBillingStatementLineItem {
    /// The amount of the line item in a single unit.
    ///
    /// This is a positive integer in the smallest currency unit, cents. If the line item should be
    /// ₱ 120.50, the amount should be 12050.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[payrex(description = "Sets the unit price for a line item in the billing statement.")]
    pub unit_price: Option<u64>,

    /// The quantity of the line item. The quantity will be multiplied by the line_item.amount to
    /// compute the final amount of the billing statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[payrex(description = "Sets the quantity for a line item in the billing statement.")]
    pub quantity: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{BillingStatementId, BillingStatementLineItemId, Timestamp};
    use serde_json;

    #[test]
    fn test_create_billing_statement_line_item_builder() {
        let params =
            CreateBillingStatementLineItem::new(BillingStatementId::new("bstm_1"), 1500, 3)
                .description("Item A");
        assert_eq!(params.billing_statement_id.as_str(), "bstm_1");
        assert_eq!(params.description, Some("Item A".to_string()));
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
            r#"{"unit_price":500,"quantity":1,"description":"Example description"}"#
        );
    }
}
