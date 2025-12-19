//! Common types and traits used across the SDK.

use serde::{Deserialize, Serialize};

/// Represents a PayRex resource.
pub trait Resource {
    /// Specifies the ID type of a resource.
    type Id;

    /// Returns the ID of the resource.
    fn id(&self) -> &Self::Id;

    /// Returns the string slice representation of the object type.
    fn object_type() -> &'static str;
}

/// Types of objects from resources received from endpoints in PayRex.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectType {
    /// Payment intent resource object
    ///
    /// [Learn more about Payment intents](https://docs.payrexhq.com/docs/api/payment_intents)
    PaymentIntent,

    /// Customer resource object
    ///
    /// [Learn more about Customers](https://docs.payrexhq.com/docs/api/customers)
    Customer,

    /// Billing statement resource object
    ///
    /// [Learn more about Billing statements](https://docs.payrexhq.com/docs/api/billing_statements)
    BillingStatement,

    /// Billing statement line item resource object
    ///
    /// [Learn more about Billing statement line items](https://docs.payrexhq.com/docs/api/billing_statement_line_items)
    BillingStatementLineItem,

    /// Checkout Session resource object.
    ///
    /// [Learn more about Checkout sessions](https://docs.payrexhq.com/docs/api/checkout_sessions)
    CheckoutSession,

    /// Payment resource object.
    ///
    /// [Learn more about Payments](https://docs.payrexhq.com/docs/api/payments)
    Payment,

    /// Refund resource object.
    ///
    /// [Learn more about Refunds](https://docs.payrexhq.com/docs/api/refunds)
    Refund,

    /// Webhook resource object.
    ///
    /// [Learn more about Webhooks](https://docs.payrexhq.com/docs/api/webhooks)
    Webhook,

    /// Event resource object.
    ///
    /// [Learn more about Events](https://docs.payrexhq.com/docs/api/events)
    Event,

    /// Payout resource object.
    ///
    /// [Learn more about Payouts](https://docs.payrexhq.com/docs/api/payouts)
    Payout,

    /// Payout transaction resource object.
    ///
    /// [Learn more about Payout transactions](https://docs.payrexhq.com/docs/api/payout_transactions)
    PayoutTransaction,

    /// List resource object to be used in pagination.
    List,
}

/// This represents a deleted resource from a delete endpoint in PayRex.
///
/// The ID represents the resource ID of a deleted resource.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Deleted<Id> {
    /// The Resource ID of a deleted resource.
    pub id: Id,

    /// Returns `true` if a resource was deleted successfully.
    pub deleted: bool,

    /// Contains the actual object/data in the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
}

impl<Id> Deleted<Id> {
    /// Creates a new [`Deleted`] instance for a resource.
    #[must_use]
    pub fn new(id: Id) -> Self {
        Self {
            id,
            deleted: true,
            object: None,
        }
    }

    /// Sets the object in a deleted resource.
    pub fn object(mut self, object: impl Into<String>) -> Self {
        self.object = Some(object.into());
        self
    }
}

/// Packs an ID and object to a single representation. This can either hold a valid PayRex ID or an
/// arbitrary object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Expandable<Id, T> {
    /// Expandable is of type ID.
    Id(Id),

    /// Expandable is an object.
    Object(Box<T>),
}

impl<Id, T> Expandable<Id, T> {
    /// Returns `true` if the [`Expandable`] is an ID.
    #[must_use]
    pub const fn is_id(&self) -> bool {
        matches!(self, Self::Id(_))
    }

    /// Returns true if the [`Expandable`] is an object.
    #[must_use]
    pub const fn is_object(&self) -> bool {
        matches!(self, Self::Object(_))
    }

    /// Returns `Some(T)` if the [`Expandable`] is an ID, otherwise it returns `None`.
    #[must_use]
    pub const fn as_id(&self) -> Option<&Id> {
        match self {
            Self::Id(id) => Some(id),
            Self::Object(_) => None,
        }
    }

    /// Returns `Some(T)` if the [`Expandable`] is an object, otherwise it returns `None`.
    #[must_use]
    pub fn as_object(&self) -> Option<&T> {
        match self {
            Self::Id(_) => None,
            Self::Object(obj) => Some(obj),
        }
    }
}

/// Represents the valid range of a query in list parameters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RangeQuery<T> {
    /// Items strictly greater than this will be included in in the result of a query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gt: Option<T>,

    /// Items greater than or equal to this will be included in the result of a query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gte: Option<T>,

    /// Items strictly less than this will be included in the result of a query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<T>,

    /// Items less than or equal to this will be included in the result of a query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lte: Option<T>,
}

impl<T> RangeQuery<T> {
    /// Creates a new empty [`RangeQuery`] instance. By default, all result is included in a query
    /// since `gt`, `gte`, `lt`, and `lte` are set to `None`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            gt: None,
            gte: None,
            lt: None,
            lte: None,
        }
    }

    /// Sets the item that should be greater than to be included in the result of a query.
    #[must_use]
    pub fn gt(mut self, value: T) -> Self {
        self.gt = Some(value);
        self
    }

    /// Sets the item that should be greater than or equal to be included in the result of a query.
    #[must_use]
    pub fn gte(mut self, value: T) -> Self {
        self.gte = Some(value);
        self
    }

    /// Sets the item that should be less than to be included in the result of a query.
    #[must_use]
    pub fn lt(mut self, value: T) -> Self {
        self.lt = Some(value);
        self
    }

    /// Sets the item that should be less than or equal to be included in the result of a query.
    #[must_use]
    pub fn lte(mut self, value: T) -> Self {
        self.lte = Some(value);
        self
    }
}

impl<T> Default for RangeQuery<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expandable_id() {
        let expandable: Expandable<String, String> = Expandable::Id("test_id".to_string());
        assert!(expandable.is_id());
        assert!(!expandable.is_object());
        assert_eq!(expandable.as_id(), Some(&"test_id".to_string()));
    }

    #[test]
    fn test_expandable_object() {
        let expandable: Expandable<String, String> =
            Expandable::Object(Box::new("test_object".to_string()));
        assert!(!expandable.is_id());
        assert!(expandable.is_object());
        assert_eq!(expandable.as_object(), Some(&"test_object".to_string()));
    }

    #[test]
    fn test_range_query() {
        let range = RangeQuery::new().gte(10).lt(100);

        assert_eq!(range.gte, Some(10));
        assert_eq!(range.lt, Some(100));
        assert_eq!(range.gt, None);
        assert_eq!(range.lte, None);
    }
}
