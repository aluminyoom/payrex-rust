//! Common types used throughout the PayRex SDK.
//!
//! This module contains shared types, traits, and utilities used across
//! different API resources.

pub mod common;
pub mod currency;
pub mod event;
pub mod ids;
pub mod metadata;
pub mod pagination;
pub mod payment_methods;
pub mod timestamp;

// Re-export commonly used types
pub use common::*;
pub use currency::Currency;
pub use ids::*;
pub use metadata::Metadata;
pub use pagination::{List, ListParams};
pub use payment_methods::*;
pub use timestamp::Timestamp;
