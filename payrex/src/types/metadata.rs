//! Metadata support for PayRex resources.
//!
//! Metadata allows you to store additional structured information on PayRex objects.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Metadata is a set of key-value pairs that you can attach to an object.
///
/// This can be useful for storing additional information about the object in a
/// structured format. You can use metadata to store things like internal IDs,
/// customer notes, or any other information you need.
///
/// # Examples
///
/// ```
/// use payrex::types::Metadata;
///
/// let mut metadata = Metadata::new();
/// metadata.insert("order_id", "12345");
/// metadata.insert("customer_note", "VIP customer");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct Metadata(HashMap<String, String>);

impl Metadata {
    #[must_use]
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    #[must_use]
    pub fn with_pair(key: impl Into<String>, value: impl Into<String>) -> Self {
        let mut metadata = Self::new();
        metadata.insert(key, value);
        metadata
    }

    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.0.insert(key.into(), value.into());
    }

    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(String::as_str)
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.0.remove(key)
    }

    #[must_use]
    pub fn contains_key(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.0.iter()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl From<HashMap<String, String>> for Metadata {
    fn from(map: HashMap<String, String>) -> Self {
        Self(map)
    }
}

impl From<Metadata> for HashMap<String, String> {
    fn from(metadata: Metadata) -> Self {
        metadata.0
    }
}

impl FromIterator<(String, String)> for Metadata {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        Self(HashMap::from_iter(iter))
    }
}

impl<'a> IntoIterator for &'a Metadata {
    type Item = (&'a String, &'a String);
    type IntoIter = std::collections::hash_map::Iter<'a, String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_new() {
        let metadata = Metadata::new();
        assert!(metadata.is_empty());
        assert_eq!(metadata.len(), 0);
    }

    #[test]
    fn test_metadata_with_pair() {
        let metadata = Metadata::with_pair("key", "value");
        assert_eq!(metadata.get("key"), Some("value"));
        assert_eq!(metadata.len(), 1);
    }

    #[test]
    fn test_metadata_insert_get() {
        let mut metadata = Metadata::new();
        metadata.insert("order_id", "12345");
        metadata.insert("customer_note", "VIP");

        assert_eq!(metadata.get("order_id"), Some("12345"));
        assert_eq!(metadata.get("customer_note"), Some("VIP"));
        assert_eq!(metadata.get("nonexistent"), None);
    }

    #[test]
    fn test_metadata_remove() {
        let mut metadata = Metadata::new();
        metadata.insert("key", "value");

        assert_eq!(metadata.remove("key"), Some("value".to_string()));
        assert_eq!(metadata.get("key"), None);
        assert!(metadata.is_empty());
    }

    #[test]
    fn test_metadata_contains_key() {
        let mut metadata = Metadata::new();
        metadata.insert("key", "value");

        assert!(metadata.contains_key("key"));
        assert!(!metadata.contains_key("nonexistent"));
    }

    #[test]
    fn test_metadata_clear() {
        let mut metadata = Metadata::new();
        metadata.insert("key1", "value1");
        metadata.insert("key2", "value2");

        metadata.clear();
        assert!(metadata.is_empty());
    }

    #[test]
    fn test_metadata_from_hashmap() {
        let mut map = HashMap::new();
        map.insert("key".to_string(), "value".to_string());

        let metadata = Metadata::from(map);
        assert_eq!(metadata.get("key"), Some("value"));
    }

    #[test]
    fn test_metadata_serialization() {
        let mut metadata = Metadata::new();
        metadata.insert("order_id", "12345");

        let json = serde_json::to_string(&metadata).unwrap();
        assert!(json.contains("order_id"));
        assert!(json.contains("12345"));
    }

    #[test]
    fn test_metadata_deserialization() {
        let json = r#"{"order_id":"12345","note":"test"}"#;
        let metadata: Metadata = serde_json::from_str(json).unwrap();

        assert_eq!(metadata.get("order_id"), Some("12345"));
        assert_eq!(metadata.get("note"), Some("test"));
    }
}
