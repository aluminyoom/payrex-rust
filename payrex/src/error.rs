//! Error types for the PayRex SDK.
//!
//! This module provides comprehensive error handling using the `thiserror` crate.
//! All errors implement `std::error::Error` and can be easily converted and propagated.

use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error: {kind} - {message}")]
    Api {
        kind: ErrorKind,
        message: String,
        status_code: Option<u16>,
        request_id: Option<String>,
    },

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Invalid API key: {0}")]
    InvalidApiKey(String),

    #[error("Rate limit exceeded. Retry after: {retry_after:?}")]
    RateLimit {
        retry_after: Option<std::time::Duration>,
    },

    #[error("Request timed out after {0:?}")]
    Timeout(std::time::Duration),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Authentication failed: {0}")]
    Authentication(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Idempotency error: {0}")]
    Idempotency(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    InvalidRequest,
    Authentication,
    RateLimit,
    NotFound,
    PermissionDenied,
    Idempotency,
    ServerError,
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

impl ErrorKind {
    #[must_use]
    pub fn from_str(s: &str) -> Self {
        match s {
            "invalid_request" | "invalid_request_error" => Self::InvalidRequest,
            "authentication" | "authentication_error" => Self::Authentication,
            "rate_limit" | "rate_limit_error" => Self::RateLimit,
            "not_found" | "resource_not_found" => Self::NotFound,
            "permission_denied" | "forbidden" => Self::PermissionDenied,
            "idempotency" | "idempotency_error" => Self::Idempotency,
            "server_error" | "internal_server_error" => Self::ServerError,
            _ => Self::Unknown,
        }
    }

    #[must_use]
    pub const fn is_retryable(self) -> bool {
        matches!(self, Self::RateLimit | Self::ServerError)
    }
}

impl Error {
    #[must_use]
    pub fn api(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self::Api {
            kind,
            message: message.into(),
            status_code: None,
            request_id: None,
        }
    }

    #[must_use]
    pub fn api_with_status(kind: ErrorKind, message: impl Into<String>, status_code: u16) -> Self {
        Self::Api {
            kind,
            message: message.into(),
            status_code: Some(status_code),
            request_id: None,
        }
    }

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

    #[must_use]
    pub const fn status_code(&self) -> Option<u16> {
        match self {
            Self::Api { status_code, .. } => *status_code,
            _ => None,
        }
    }

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
            ErrorKind::from_str("invalid_request"),
            ErrorKind::InvalidRequest
        );
        assert_eq!(
            ErrorKind::from_str("authentication_error"),
            ErrorKind::Authentication
        );
        assert_eq!(ErrorKind::from_str("rate_limit"), ErrorKind::RateLimit);
        assert_eq!(ErrorKind::from_str("unknown"), ErrorKind::Unknown);
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
