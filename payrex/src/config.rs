//! Configuration for the PayRex client.
//!
//! This module provides configuration options for customizing the behavior
//! of the PayRex client, including timeouts, retries, and API endpoints.

use crate::{API_BASE_URL, Error, Result};
use std::time::Duration;

/// Configuration for the PayRex client.
///
/// Use [`ConfigBuilder`] to construct a configuration with custom settings.
#[derive(Debug, Clone)]
pub struct Config {
    pub(crate) api_key: String,
    pub(crate) api_base_url: String,
    pub(crate) timeout: Duration,
    pub(crate) max_retries: u32,
    pub(crate) retry_delay: Duration,
    pub(crate) user_agent: String,
    pub(crate) test_mode: bool,
}

impl Config {
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        let api_key = api_key.into();

        if api_key.is_empty() {
            return Err(Error::InvalidApiKey("API key cannot be empty".to_string()));
        }

        let test_mode = api_key.starts_with("sk_test_");

        Ok(Self {
            api_key,
            api_base_url: API_BASE_URL.to_string(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
            retry_delay: Duration::from_millis(500),
            user_agent: format!("payrex-rust/{}", crate::VERSION),
            test_mode,
        })
    }

    #[must_use]
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    #[must_use]
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    #[must_use]
    pub fn api_base_url(&self) -> &str {
        &self.api_base_url
    }

    #[must_use]
    pub const fn timeout(&self) -> Duration {
        self.timeout
    }

    #[must_use]
    pub const fn max_retries(&self) -> u32 {
        self.max_retries
    }

    #[must_use]
    pub const fn retry_delay(&self) -> Duration {
        self.retry_delay
    }

    #[must_use]
    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }

    #[must_use]
    pub const fn is_test_mode(&self) -> bool {
        self.test_mode
    }
}

/// Builder for [`Config`].
///
/// Provides a fluent interface for constructing a configuration with custom settings.
#[derive(Debug, Default)]
pub struct ConfigBuilder {
    api_key: Option<String>,
    api_base_url: Option<String>,
    timeout: Option<Duration>,
    max_retries: Option<u32>,
    retry_delay: Option<Duration>,
    user_agent: Option<String>,
    test_mode: bool,
}

impl ConfigBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    #[must_use]
    pub fn api_base_url(mut self, url: impl Into<String>) -> Self {
        self.api_base_url = Some(url.into());
        self
    }

    #[must_use]
    pub const fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    #[must_use]
    pub const fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    #[must_use]
    pub const fn retry_delay(mut self, delay: Duration) -> Self {
        self.retry_delay = Some(delay);
        self
    }

    #[must_use]
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    #[must_use]
    pub const fn test_mode(mut self, enabled: bool) -> Self {
        self.test_mode = enabled;
        self
    }

    pub fn build(self) -> Result<Config> {
        let api_key = self
            .api_key
            .ok_or_else(|| Error::Config("API key is required".to_string()))?;

        if api_key.is_empty() {
            return Err(Error::InvalidApiKey("API key cannot be empty".to_string()));
        }

        let test_mode = self.test_mode || api_key.starts_with("sk_test_");

        Ok(Config {
            api_key,
            api_base_url: self
                .api_base_url
                .unwrap_or_else(|| API_BASE_URL.to_string()),
            timeout: self.timeout.unwrap_or(Duration::from_secs(30)),
            max_retries: self.max_retries.unwrap_or(3),
            retry_delay: self.retry_delay.unwrap_or(Duration::from_millis(500)),
            user_agent: self
                .user_agent
                .unwrap_or_else(|| format!("payrex-rust/{}", crate::VERSION)),
            test_mode,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new() {
        let config = Config::new("test_key").unwrap();
        assert_eq!(config.api_key(), "test_key");
        assert_eq!(config.timeout(), Duration::from_secs(30));
        assert_eq!(config.max_retries(), 3);
    }

    #[test]
    fn test_config_new_empty_key() {
        let result = Config::new("");
        assert!(result.is_err());
    }

    #[test]
    fn test_config_builder() {
        let config = Config::builder()
            .api_key("test_key")
            .timeout(Duration::from_secs(60))
            .max_retries(5)
            .test_mode(true)
            .build()
            .unwrap();

        assert_eq!(config.api_key(), "test_key");
        assert_eq!(config.timeout(), Duration::from_secs(60));
        assert_eq!(config.max_retries(), 5);
        assert!(config.is_test_mode());
    }

    #[test]
    fn test_config_builder_missing_api_key() {
        let result = Config::builder().timeout(Duration::from_secs(60)).build();

        assert!(result.is_err());
    }

    #[test]
    fn test_auto_detect_test_mode() {
        let test_config = Config::new("sk_test_abc123").unwrap();
        assert!(test_config.is_test_mode());

        let live_config = Config::new("sk_live_abc123").unwrap();
        assert!(!live_config.is_test_mode());

        let other_config = Config::new("some_other_key").unwrap();
        assert!(!other_config.is_test_mode());
    }

    #[test]
    fn test_auto_detect_test_mode_builder() {
        let test_config = Config::builder().api_key("sk_test_abc123").build().unwrap();
        assert!(test_config.is_test_mode());

        let live_config = Config::builder().api_key("sk_live_abc123").build().unwrap();
        assert!(!live_config.is_test_mode());

        // Explicit test_mode(true) overrides auto-detection
        let explicit_config = Config::builder()
            .api_key("sk_live_abc123")
            .test_mode(true)
            .build()
            .unwrap();
        assert!(explicit_config.is_test_mode());
    }
}
