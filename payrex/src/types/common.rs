//! Common types and traits used across the SDK.

use serde::{Deserialize, Serialize};

pub trait Resource {
    type Id;
    fn id(&self) -> &Self::Id;
    fn object_type() -> &'static str;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectType {
    PaymentIntent,
    Customer,
    BillingStatement,
    BillingStatementLineItem,
    CheckoutSession,
    Payment,
    Refund,
    Webhook,
    Event,
    Payout,
    PayoutTransaction,
    List,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Deleted<Id> {
    pub id: Id,
    pub deleted: bool,
    pub object: String,
}

impl<Id> Deleted<Id> {
    #[must_use]
    pub fn new(id: Id, object: String) -> Self {
        Self {
            id,
            deleted: true,
            object,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Expandable<Id, T> {
    Id(Id),
    Object(Box<T>),
}

impl<Id, T> Expandable<Id, T> {
    #[must_use]
    pub const fn is_id(&self) -> bool {
        matches!(self, Self::Id(_))
    }

    #[must_use]
    pub const fn is_object(&self) -> bool {
        matches!(self, Self::Object(_))
    }

    #[must_use]
    pub const fn as_id(&self) -> Option<&Id> {
        match self {
            Self::Id(id) => Some(id),
            Self::Object(_) => None,
        }
    }

    #[must_use]
    pub fn as_object(&self) -> Option<&T> {
        match self {
            Self::Id(_) => None,
            Self::Object(obj) => Some(obj),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RangeQuery<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gt: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gte: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lte: Option<T>,
}

impl<T> RangeQuery<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            gt: None,
            gte: None,
            lt: None,
            lte: None,
        }
    }

    #[must_use]
    pub fn gt(mut self, value: T) -> Self {
        self.gt = Some(value);
        self
    }

    #[must_use]
    pub fn gte(mut self, value: T) -> Self {
        self.gte = Some(value);
        self
    }

    #[must_use]
    pub fn lt(mut self, value: T) -> Self {
        self.lt = Some(value);
        self
    }

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
