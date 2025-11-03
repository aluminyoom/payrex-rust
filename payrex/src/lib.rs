//! Unofficial Rust SDK for PayRex payment platform.
//!
//! This is a work in progress SDK with foundation complete but API implementations pending.
//!
//! The SDK automatically detects test mode from your API key and sets the `test_mode` flag accordingly.
//! You can check if you're in test mode using `client.config().is_test_mode()`.

#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

// Core modules
mod client;
mod config;
mod error;
mod http;

// Type modules
pub mod types;

// Resource modules
pub mod resources;

// Re-exports
pub use client::Client;
pub use config::{Config, ConfigBuilder};
pub use error::{Error, ErrorKind, Result};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const API_BASE_URL: &str = "https://api.payrexhq.com";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_exists() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_api_url_valid() {
        assert!(API_BASE_URL.starts_with("https://"));
    }
}
