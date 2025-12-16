//! Types for payment methods, card options and capture methods.

use serde::{Deserialize, Serialize};

/// Available payment methods for a [`PaymentIntent`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentMethod {
    /// Card payments
    #[serde(rename = "card")]
    Card,

    /// GCash payments
    #[serde(rename = "gcash")]
    GCash,

    /// Maya payments
    #[serde(rename = "maya")]
    Maya,

    /// QRPH payments
    #[serde(rename = "qrph")]
    QRPh,
}

impl PaymentMethod {
    /// Returns the string representation of the payment method.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Card => "card",
            Self::GCash => "gcash",
            Self::Maya => "maya",
            Self::QRPh => "qrph",
        }
    }
}

/// A set of key-value pairs that can modify the behavior of the payment method attached to the
/// payment intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaymentMethodOptions {
    /// Hash of options for the `card` payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardOptions>,
}

/// Hash of options for the `card` payment method.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardOptions {
    /// Describes the `capture_type` of a card payment. Possible values are `automatic` or
    /// `manual`. This is used for hold then capture feature. Please refer to this
    /// [guide](https://docs.payrexhq.com/docs/guide/developer_handbook/payments/payment_methods/card/hold_then_capture)
    /// for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_type: Option<CaptureMethod>,

    /// Restricts the allowed card BINs for a card payment. Please refer to this
    /// [guide](https://docs.payrexhq.com/docs/guide/developer_handbook/payments/payment_methods/card/allowed_bins)
    /// for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_bins: Option<Vec<String>>,

    /// Restricts the allowed card funding for a card payment. Please refer to this
    /// [guide](https://docs.payrexhq.com/docs/guide/developer_handbook/payments/payment_methods/card/allowed_funding)
    /// for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_funding: Option<Vec<String>>,
}

/// Describes the `capture_method` of a card payment. Possible values are `automatic` or
/// `manual`. This is used for hold then capture feature. Please refer to this
/// [guide](https://docs.payrexhq.com/docs/guide/developer_handbook/payments/payment_methods/card/hold_then_capture)
/// for more details.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureMethod {
    /// The payment is captured automatically.
    Automatic,

    /// The payment requires manual capture.
    Manual,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capture_method_serialization() {
        use serde_json;

        let capture_method = CaptureMethod::Automatic;
        let json = serde_json::to_string(&capture_method).unwrap();
        assert_eq!(json, "\"automatic\"");

        let capture_method = CaptureMethod::Manual;
        let json = serde_json::to_string(&capture_method).unwrap();
        assert_eq!(json, "\"manual\"");
    }

    #[test]
    fn test_payment_method_serialization() {
        use PaymentMethod::*;
        use serde_json;

        // Test individual payment methods
        let method = Card;
        let json = serde_json::to_string(&method).unwrap();
        assert_eq!(json, "\"card\"");

        let method = GCash;
        let json = serde_json::to_string(&method).unwrap();
        assert_eq!(json, "\"gcash\"");

        let method = Maya;
        let json = serde_json::to_string(&method).unwrap();
        assert_eq!(json, "\"maya\"");

        let method = QRPh;
        let json = serde_json::to_string(&method).unwrap();
        assert_eq!(json, "\"qrph\"");

        // Test as_str method
        assert_eq!(Card.as_str(), "card");
        assert_eq!(GCash.as_str(), "gcash");
        assert_eq!(Maya.as_str(), "maya");
        assert_eq!(QRPh.as_str(), "qrph");
    }
}
