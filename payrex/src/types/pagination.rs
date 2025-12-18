//! Pagination support for list endpoints.
//!
//! PayRex uses cursor-based pagination for list endpoints.

use serde::{Deserialize, Serialize};

/// Represents the collection for list parameters used in list endpoints in the API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct List<T> {
    /// Type of object that the current list holds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,

    /// Contains a collection of data stored in a list.
    pub data: Vec<T>,

    /// Indicates whether the current pagination has more items in the list.
    pub has_more: bool,

    /// Indicates the next page in a list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,

    /// The total number of elements in a list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<u64>,
}

impl<T> List<T> {
    /// Instantiates a new list.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            object: Some("list".to_string()),
            data: Vec::new(),
            has_more: false,
            next_page: None,
            total_count: Some(0),
        }
    }

    /// Returns `true` if a list is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the number of items in a list.
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns the iterator of data from a list.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

/// Baseline list parameters for list endpoints
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ListParams {
    /// Limits the number of resources returned by the endpoint. The minimum amount is 1, and the
    /// maximum is 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    //#[payrex(description = "Sets the limit for a pagination in a list.")]
    pub limit: Option<u32>,

    /// A cursor used in pagination. `after` is a resource ID that defines your place in the list.
    /// For example, if you call a list request and receive 10 resources ending with `bstm_1234`,
    /// your subsequent calls can include `after=bstm_1234` to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    //#[payrex(description = "Sets the page number to search after for in a list.")]
    pub after: Option<String>,

    /// A cursor used in pagination. before is a resource ID that defines your place in the list.
    /// For example, if you call a list request and receive 10 resources, starting with `bstm_1234`,
    /// your subsequent calls can include `before=bstm_1234` to fetch the list's previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    //#[payrex(description = "Sets the page number to search before in a list.")]
    pub before: Option<String>,
}

impl ListParams {
    /// Creates a new [`ListParams`] instance.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            limit: None,
            after: None,
            before: None,
        }
    }

    /// Sets the limit to number of resources returned by an endpoint (Clamped to 100).
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit.clamp(1, 100));
        self
    }

    /// Sets the resource ID in which resources are fetched after.
    #[must_use]
    pub fn after(mut self, id: impl Into<String>) -> Self {
        self.after = Some(id.into());
        self
    }

    /// Sets the resource ID in which resources are fetched before.
    #[must_use]
    pub fn before(mut self, id: impl Into<String>) -> Self {
        self.before = Some(id.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_empty() {
        let list: List<String> = List::empty();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
        assert!(!list.has_more);
    }

    #[test]
    fn test_list_with_data() {
        let list = List {
            object: Some("list".to_string()),
            data: vec!["item1".to_string(), "item2".to_string()],
            has_more: true,
            next_page: Some("next_url".to_string()),
            total_count: Some(10),
        };

        assert!(!list.is_empty());
        assert_eq!(list.len(), 2);
        assert!(list.has_more);
        assert_eq!(list.total_count, Some(10));
    }

    #[test]
    fn test_list_iteration() {
        let list = List {
            object: Some("list".to_string()),
            data: vec![1, 2, 3],
            has_more: false,
            next_page: None,
            total_count: Some(3),
        };

        let items: Vec<_> = list.iter().copied().collect();
        assert_eq!(items, vec![1, 2, 3]);
    }

    #[test]
    fn test_list_into_iter() {
        let list = List {
            object: Some("list".to_string()),
            data: vec![1, 2, 3],
            has_more: false,
            next_page: None,
            total_count: Some(3),
        };

        let items: Vec<_> = list.into_iter().collect();
        assert_eq!(items, vec![1, 2, 3]);
    }

    #[test]
    fn test_list_params() {
        let params = ListParams::new().limit(50).after("obj_123");

        assert_eq!(params.limit, Some(50));
        assert_eq!(params.after, Some("obj_123".to_string()));
    }

    #[test]
    fn test_list_params_limit_clamping() {
        let params = ListParams::new().limit(200);
        assert_eq!(params.limit, Some(100)); // Should be clamped to 100

        let params = ListParams::new().limit(0);
        assert_eq!(params.limit, Some(1)); // Should be clamped to 1
    }

    #[test]
    fn test_list_serialization() {
        let list = List {
            object: Some("list".to_string()),
            data: vec![1, 2, 3],
            has_more: false,
            next_page: None,
            total_count: Some(3),
        };

        let json = serde_json::to_string(&list).unwrap();
        assert!(json.contains("\"object\":\"list\""));
        assert!(json.contains("\"data\":[1,2,3]"));
    }
}
