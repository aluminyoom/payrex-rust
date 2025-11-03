//! API resource modules.
//!
//! This module contains all the API resource implementations for interacting
//! with different parts of the PayRex API.

pub mod billing_statement_line_items;
pub mod billing_statements;
pub mod checkout_sessions;
pub mod customers;
pub mod payment_intents;
pub mod payments;
pub mod payouts;
pub mod refunds;
pub mod webhooks;

// Re-export resource types
pub use billing_statement_line_items::BillingStatementLineItems;
pub use billing_statements::BillingStatements;
pub use checkout_sessions::CheckoutSessions;
pub use customers::Customers;
pub use payment_intents::PaymentIntents;
pub use payments::Payments;
pub use payouts::Payouts;
pub use refunds::Refunds;
pub use webhooks::Webhooks;
