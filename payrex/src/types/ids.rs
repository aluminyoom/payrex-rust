//! Strongly-typed IDs for PayRex resources.
//!
//! Each resource type has its own ID type to prevent mixing IDs between resources.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Macro to define a strongly-typed ID.
macro_rules! define_id {
    ($name:ident, $prefix:expr, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            /// # Panics
            ///
            /// Panics if the ID doesn't start with the expected prefix.
            #[must_use]
            pub fn new(id: impl Into<String>) -> Self {
                let id = id.into();
                assert!(
                    id.starts_with($prefix),
                    "Invalid {} ID: expected prefix '{}', got '{}'",
                    stringify!($name),
                    $prefix,
                    id
                );
                Self(id)
            }

            #[must_use]
            pub fn new_unchecked(id: impl Into<String>) -> Self {
                Self(id.into())
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }

            #[must_use]
            pub const fn prefix() -> &'static str {
                $prefix
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl From<$name> for String {
            fn from(id: $name) -> Self {
                id.0
            }
        }
    };
}

// Define ID types for each resource
define_id!(PaymentIntentId, "pi_", "Payment Intent ID");
define_id!(CustomerId, "cus_", "Customer ID");
define_id!(BillingStatementId, "bstm_", "Billing Statement ID");
define_id!(
    BillingStatementLineItemId,
    "bstm_li_",
    "Billing Statement Line Item ID"
);
define_id!(CheckoutSessionId, "cs_", "Checkout Session ID");
define_id!(
    CheckoutSessionLineItemId,
    "cs_li_",
    "Checkout Session Line Item ID"
);
define_id!(PaymentId, "pay_", "Payment ID");
define_id!(RefundId, "re_", "Refund ID");
define_id!(WebhookId, "wh_", "Webhook ID");
define_id!(EventId, "evt_", "Event ID");
define_id!(PayoutId, "po_", "Payout ID");
define_id!(PayoutTransactionId, "pot_", "Payout Transaction ID");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payment_intent_id() {
        let id = PaymentIntentId::new("pi_123456");
        assert_eq!(id.as_str(), "pi_123456");
        assert_eq!(id.to_string(), "pi_123456");
    }

    #[test]
    #[should_panic(expected = "Invalid PaymentIntentId ID")]
    fn test_payment_intent_id_invalid_prefix() {
        let _ = PaymentIntentId::new("invalid_123456");
    }

    #[test]
    fn test_customer_id() {
        let id = CustomerId::new("cus_123456");
        assert_eq!(id.as_str(), "cus_123456");
    }

    #[test]
    fn test_id_serialization() {
        let id = PaymentIntentId::new("pi_123456");
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"pi_123456\"");
    }

    #[test]
    fn test_id_deserialization() {
        let json = "\"pi_123456\"";
        let id: PaymentIntentId = serde_json::from_str(json).unwrap();
        assert_eq!(id.as_str(), "pi_123456");
    }
}
