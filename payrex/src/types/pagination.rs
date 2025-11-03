//! Pagination support for list endpoints.
//!
//! PayRex uses cursor-based pagination for list endpoints.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct List<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    pub data: Vec<T>,
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<u64>,
}

impl<T> List<T> {
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

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl ListParams {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            limit: None,
            after: None,
            before: None,
        }
    }

    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit.clamp(1, 100));
        self
    }

    #[must_use]
    pub fn after(mut self, id: impl Into<String>) -> Self {
        self.after = Some(id.into());
        self
    }

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
