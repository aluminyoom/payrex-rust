# payrex-rust

> [!CAUTION]
> This SDK is currently in early development. The foundation is complete, but API implementations are stubs. Not ready for production use yet!

Unofficial Rust SDK for [PayRex](https://payrexhq.com)

## Roadmap

- [x] Foundations
- [x] APIs
  - [x] Payment Intents
  - [x] Payments
  - [x] Customers
  - [x] Billing Statements
  - [x] Billing Statement Line Items
  - [x] Checkout Sessions
  - [x] Webhooks
  - [x] Events
  - [x] Refunds
  - [x] Payouts
- [ ] Final
  - [x] Integration tests
  - [x] Documentation improvements
  - [ ] Publish to crates.io

## Installation

For now, you can use it from git:

```toml
[dependencies]
payrex = { git = "https://github.com/mayspc/payrex-rust" }
tokio = { version = "1", features = ["full"] }
```

## Quick Start

> [!NOTE]
> These examples show the intended API design. Currently, all methods return stub/placeholder data.

```rust
use payrex::{Client, Currency};
use payrex::resources::payment_intents::{CreatePaymentIntent, CaptureMethod, PaymentMethod};

#[tokio::main]
async fn main() -> Result<(), payrex::Error> {
    // Initialize the client with your API key
    let client = Client::new("your_secret_key");

    // Create a payment intent
    use PaymentMethod::*;
    let payment_methods = &[Card, Gcash];
    let params = CreatePaymentIntent::new(10000, Currency::PHP, payment_methods)
        .description("Order #12345")
        .capture_method(CaptureMethod::Automatic);

    let payment_intent = client.payment_intents().create(params).await?;

    println!("Created payment intent: {}", payment_intent.id);
    println!("Status: {:?}", payment_intent.status);

    Ok(())
}
```

## Configuration

```rust
use payrex::{Client, Config};
use std::time::Duration;

let config = Config::builder()
    .api_key("your_secret_key")
    .timeout(Duration::from_secs(30))
    .max_retries(3)
    .test_mode(true)
    .build()?;

let client = Client::with_config(config)?;
```

## Testing

The SDK will include comprehensive tests. Run them with:

```bash
cargo test
```

## Examples

See the [examples](examples/) directory for more usage examples:

- `basic_usage.rs` - Basic payment intent creation

## Requirements

- Rust 1.90 or later

## Contributing

Contributions are welcome! This is an early-stage project, so there's plenty to do.

Feel free to open issues or submit PRs!

---

Built for the Rust and PayRex communities.
