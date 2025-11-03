//! Main client for interacting with the PayRex API.
//!
//! The [`Client`] provides access to all PayRex API resources and handles
//! authentication, request/response processing, and error handling.

use crate::{
    Result,
    config::Config,
    http::HttpClient,
    resources::{
        BillingStatementLineItems, BillingStatements, CheckoutSessions, Customers, PaymentIntents,
        Payments, Payouts, Refunds, Webhooks,
    },
};
use std::sync::Arc;

/// Main client for the PayRex API.
///
/// The client provides access to all API resources and manages authentication
/// and HTTP communication with the PayRex API.
///
/// # Examples
///
/// ```rust,no_run
/// use payrex::Client;
///
/// #[tokio::main]
/// async fn main() -> Result<(), payrex::Error> {
///     let client = Client::new("your_secret_key");
///
///     // Access API resources
///     // let payment = client.payment_intents().create(...).await?;
///
///     Ok(())
/// }
/// ```
#[derive(Clone)]
pub struct Client {
    http: Arc<HttpClient>,
}

impl Client {
    /// # Panics
    ///
    /// Panics if the API key is invalid. For fallible construction, use [`Client::try_new`].
    #[must_use]
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::try_new(api_key).expect("Failed to create PayRex client")
    }

    pub fn try_new(api_key: impl Into<String>) -> Result<Self> {
        let config = Config::new(api_key)?;
        Self::with_config(config)
    }

    pub fn with_config(config: Config) -> Result<Self> {
        let http = HttpClient::new(config)?;
        Ok(Self {
            http: Arc::new(http),
        })
    }

    #[must_use]
    pub fn payment_intents(&self) -> PaymentIntents {
        PaymentIntents::new(Arc::clone(&self.http))
    }

    #[must_use]
    pub fn customers(&self) -> Customers {
        Customers::new(Arc::clone(&self.http))
    }

    #[must_use]
    pub fn billing_statements(&self) -> BillingStatements {
        BillingStatements::new(Arc::clone(&self.http))
    }

    #[must_use]
    pub fn billing_statement_line_items(&self) -> BillingStatementLineItems {
        BillingStatementLineItems::new(Arc::clone(&self.http))
    }

    #[must_use]
    pub fn checkout_sessions(&self) -> CheckoutSessions {
        CheckoutSessions::new(Arc::clone(&self.http))
    }

    #[must_use]
    pub fn refunds(&self) -> Refunds {
        Refunds::new(Arc::clone(&self.http))
    }

    #[must_use]
    pub fn webhooks(&self) -> Webhooks {
        Webhooks::new(Arc::clone(&self.http))
    }

    #[must_use]
    pub fn payments(&self) -> Payments {
        Payments::new(Arc::clone(&self.http))
    }

    #[must_use]
    pub fn payouts(&self) -> Payouts {
        Payouts::new(Arc::clone(&self.http))
    }
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("http", &"HttpClient { ... }")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_new() {
        let client = Client::new("test_key");
        assert!(std::sync::Arc::strong_count(&client.http) == 1);
    }

    #[test]
    fn test_client_try_new() {
        let result = Client::try_new("test_key");
        assert!(result.is_ok());
    }

    #[test]
    fn test_client_try_new_empty_key() {
        let result = Client::try_new("");
        assert!(result.is_err());
    }

    #[test]
    fn test_client_with_config() {
        let config = Config::new("test_key").unwrap();
        let result = Client::with_config(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_client_clone() {
        let client = Client::new("test_key");
        let cloned = client.clone();

        assert!(std::sync::Arc::ptr_eq(&client.http, &cloned.http));
    }
}
