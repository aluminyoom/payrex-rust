//! Error types for the PayRex SDK.
//!
//! This module provides comprehensive error handling using the `thiserror` crate.
//! All errors implement `std::error::Error` and can be easily converted and propagated.

use std::{fmt, str::FromStr};

#[allow(missing_docs)]
pub type Result<T> = std::result::Result<T, Error>;

/// The different error variants from a potentially failed API request.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The HTTP request failed for an API call. This could potentially be from wrong internal
    /// implementation or outdated HTTP client.
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// API error response from PayRex API.
    #[error("API error: {kind} - {message}")]
    Api {
        /// The type of error returned from a response.
        kind: ErrorKind,

        /// Error message from a response.
        message: String,

        /// The HTTP error status code provided by the API.
        status_code: Option<u16>,

        /// The Request ID attached on the request headers while calling PayRex API.
        request_id: Option<String>,
    },

    /// JSON encoding/decoding error response.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Configuration error from the user.
    #[error("Configuration error: {0}")]
    Config(String),

    /// This error is thrown when the user provides an invalid API key (whether it's the test or
    /// live mode).
    #[error("Invalid API key: {0}")]
    InvalidApiKey(String),

    /// Rate limit error occurs when too many request was sent at a time.
    #[error("Rate limit exceeded. Retry after: {retry_after:?}")]
    RateLimit {
        /// Duration to wait before retrying an API call.
        retry_after: Option<std::time::Duration>,
    },

    /// The request timed out possibly due to downtime from PayRex server or slow internet
    /// connection.
    #[error("Request timed out after {0:?}")]
    Timeout(std::time::Duration),

    /// Internal error when a request is invalid.
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    /// Occurs when a resource was not found from PayRex's server (error 404).
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Occurs when an authentication fails but with correct API key. This possibly due to downtime
    /// from PayRex's authentication servers.
    #[error("Authentication failed: {0}")]
    Authentication(String),

    /// Error when the user does not have a permission to modify a resource via the PayRex API.
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Occurs when the same request is a duplicate of the previous one.
    #[error("Idempotency error: {0}")]
    Idempotency(String),

    /// Fallback error type. This is mostly for internal errors.
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Types of errors that could occur.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// The request made was invalid either due to request body or invalid endpoint.
    InvalidRequest,

    /// The error is related to authentication.
    Authentication,

    /// Rate limit error occurs when too many request was sent at a time.
    RateLimit,

    /// Occurs when a resource was not found from PayRex's server (error 404).
    NotFound,

    /// Error when the user does not have a permission to modify a resource via the PayRex API.
    PermissionDenied,

    /// Occurs when the same request is a duplicate of the previous one.
    Idempotency,

    /// PayRex servers are down during the requests.
    ServerError,

    /// Fallback error.
    Unknown,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRequest => write!(f, "invalid_request"),
            Self::Authentication => write!(f, "authentication_error"),
            Self::RateLimit => write!(f, "rate_limit"),
            Self::NotFound => write!(f, "not_found"),
            Self::PermissionDenied => write!(f, "permission_denied"),
            Self::Idempotency => write!(f, "idempotency_error"),
            Self::ServerError => write!(f, "server_error"),
            Self::Unknown => write!(f, "unknown_error"),
        }
    }
}

impl FromStr for ErrorKind {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let err = match s {
            "invalid_request" | "invalid_request_error" => Self::InvalidRequest,
            "authentication" | "authentication_error" => Self::Authentication,
            "rate_limit" | "rate_limit_error" => Self::RateLimit,
            "not_found" | "resource_not_found" => Self::NotFound,
            "permission_denied" | "forbidden" => Self::PermissionDenied,
            "idempotency" | "idempotency_error" => Self::Idempotency,
            "server_error" | "internal_server_error" => Self::ServerError,
            _ => Self::Unknown,
        };

        Ok(err)
    }
}

impl ErrorKind {
    /// Returns `true` if the requests is retryable by checking if the error type is either
    /// [`ErrorKind::RateLimit`] or [`ErrorKind::ServerError`].
    #[must_use]
    pub const fn is_retryable(self) -> bool {
        matches!(self, Self::RateLimit | Self::ServerError)
    }
}

impl Error {
    /// Creates a new API Error instance without a status code.
    #[must_use]
    pub fn api(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self::Api {
            kind,
            message: message.into(),
            status_code: None,
            request_id: None,
        }
    }

    /// Creates a new API Error instance with a corresponding status code.
    #[must_use]
    pub fn api_with_status(kind: ErrorKind, message: impl Into<String>, status_code: u16) -> Self {
        Self::Api {
            kind,
            message: message.into(),
            status_code: Some(status_code),
            request_id: None,
        }
    }

    /// Returns `true` if the requests is retryable. This is for the [`Error`] type, use the same
    /// function in [`ErrorKind`] if you just want to check from the API error response.
    #[must_use]
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Api { kind, .. } => kind.is_retryable(),
            Self::RateLimit { .. } => true,
            Self::Timeout(_) => true,
            Self::Http(e) => e.is_timeout() || e.is_connect(),
            _ => false,
        }
    }

    /// Returns the status code of an API error. If it's not an API error, this will return `None`
    /// instead.
    #[must_use]
    pub const fn status_code(&self) -> Option<u16> {
        match self {
            Self::Api { status_code, .. } => *status_code,
            _ => None,
        }
    }

    /// Returns the request ID of an API error. If it's not an API error, this will return `None`
    /// instead.
    #[must_use]
    pub fn request_id(&self) -> Option<&str> {
        match self {
            Self::Api { request_id, .. } => request_id.as_deref(),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_kind_from_str() {
        assert_eq!(
            ErrorKind::from_str("invalid_request").unwrap(),
            ErrorKind::InvalidRequest
        );
        assert_eq!(
            ErrorKind::from_str("authentication_error").unwrap(),
            ErrorKind::Authentication
        );
        assert_eq!(
            ErrorKind::from_str("rate_limit").unwrap(),
            ErrorKind::RateLimit
        );
        assert_eq!(ErrorKind::from_str("unknown").unwrap(), ErrorKind::Unknown);
    }

    #[test]
    fn test_error_kind_is_retryable() {
        assert!(ErrorKind::RateLimit.is_retryable());
        assert!(ErrorKind::ServerError.is_retryable());
        assert!(!ErrorKind::InvalidRequest.is_retryable());
        assert!(!ErrorKind::Authentication.is_retryable());
    }

    #[test]
    fn test_error_is_retryable() {
        let error = Error::api(ErrorKind::RateLimit, "Too many requests");
        assert!(error.is_retryable());

        let error = Error::api(ErrorKind::InvalidRequest, "Bad request");
        assert!(!error.is_retryable());
    }

    #[test]
    fn test_error_status_code() {
        let error = Error::api_with_status(ErrorKind::NotFound, "Not found", 404);
        assert_eq!(error.status_code(), Some(404));
    }
}
