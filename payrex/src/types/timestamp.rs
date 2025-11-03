//! Timestamp handling for the PayRex SDK.
//!
//! PayRex API uses Unix timestamps (seconds since epoch) for all date/time values.

use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// A Unix timestamp representing seconds since the Unix epoch.
///
/// This type wraps a `DateTime<Utc>` and provides serialization/deserialization
/// to/from Unix timestamps as used by the PayRex API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    #[must_use]
    pub fn from_unix(seconds: i64) -> Self {
        Self(Utc.timestamp_opt(seconds, 0).unwrap())
    }

    #[must_use]
    pub fn now() -> Self {
        Self(Utc::now())
    }

    #[must_use]
    pub fn as_unix(&self) -> i64 {
        self.0.timestamp()
    }

    #[must_use]
    pub const fn as_datetime(&self) -> &DateTime<Utc> {
        &self.0
    }

    #[must_use]
    pub const fn to_datetime(self) -> DateTime<Utc> {
        self.0
    }

    #[must_use]
    pub fn to_rfc3339(&self) -> String {
        self.0.to_rfc3339()
    }
}

impl From<DateTime<Utc>> for Timestamp {
    fn from(dt: DateTime<Utc>) -> Self {
        Self(dt)
    }
}

impl From<Timestamp> for DateTime<Utc> {
    fn from(ts: Timestamp) -> Self {
        ts.0
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_rfc3339())
    }
}

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(self.as_unix())
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let seconds = i64::deserialize(deserializer)?;
        Ok(Self::from_unix(seconds))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_from_unix() {
        let ts = Timestamp::from_unix(1609459200); // 2021-01-01 00:00:00 UTC
        assert_eq!(ts.as_unix(), 1609459200);
    }

    #[test]
    fn test_timestamp_now() {
        let ts = Timestamp::now();
        let now = Utc::now();

        // Should be within 1 second
        assert!((ts.as_unix() - now.timestamp()).abs() <= 1);
    }

    #[test]
    fn test_timestamp_to_datetime() {
        let ts = Timestamp::from_unix(1609459200);
        let dt = ts.to_datetime();
        assert_eq!(dt.timestamp(), 1609459200);
    }

    #[test]
    fn test_timestamp_serialization() {
        let ts = Timestamp::from_unix(1609459200);
        let json = serde_json::to_string(&ts).unwrap();
        assert_eq!(json, "1609459200");
    }

    #[test]
    fn test_timestamp_deserialization() {
        let json = "1609459200";
        let ts: Timestamp = serde_json::from_str(json).unwrap();
        assert_eq!(ts.as_unix(), 1609459200);
    }

    #[test]
    fn test_timestamp_ordering() {
        let ts1 = Timestamp::from_unix(1000);
        let ts2 = Timestamp::from_unix(2000);

        assert!(ts1 < ts2);
        assert!(ts2 > ts1);
    }

    #[test]
    fn test_timestamp_display() {
        let ts = Timestamp::from_unix(1609459200);
        let display = format!("{ts}");
        assert!(display.contains("2021"));
    }
}
