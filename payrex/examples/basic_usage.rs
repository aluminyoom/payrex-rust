//! Basic usage example for the PayRex SDK.
//!
//! This example demonstrates how to:
//! - Initialize the PayRex client
//! - Create a payment intent
//! - Retrieve a payment intent
//! - Handle errors
//!
//! Run with: cargo run --example basic_usage

use payrex::resources::payment_intents::CreatePaymentIntent;
use payrex::types::{CaptureMethod, Currency, Metadata, PaymentMethod};
use payrex::{Client, Error, ErrorKind};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        std::env::var("PAYREX_API_KEY").unwrap_or_else(|_| "your_secret_api_key".to_string());

    let client = Client::new(api_key);

    println!("PayRex SDK Example - Basic Usage\n");
    println!("1. Creating a payment intent...");

    let mut metadata = Metadata::new();
    metadata.insert("order_id", "ORDER-12345");
    metadata.insert("customer_email", "customer@example.com");

    use PaymentMethod::*;
    let payment_methods = &[Card, GCash, Maya];

    let params = CreatePaymentIntent::new(10000, Currency::PHP, payment_methods)
        .description("Example payment for Order #12345")
        .capture_method(CaptureMethod::Automatic)
        .metadata(metadata);

    match client.payment_intents().create(params).await {
        Ok(payment_intent) => {
            println!("Success: Payment intent created successfully!");
            println!("  ID: {}", payment_intent.id);
            println!(
                "  Amount: {}",
                Currency::PHP.format_amount(payment_intent.amount)
            );
            println!("  Status: {:?}", payment_intent.status);
            println!("  Created: {}", payment_intent.created_at);

            println!("\n2. Retrieving the payment intent...");
            match client.payment_intents().retrieve(&payment_intent.id).await {
                Ok(retrieved) => {
                    println!("Success: Payment intent retrieved successfully!");
                    println!("  Status: {:?}", retrieved.status);
                }
                Err(e) => {
                    println!("Error: Failed to retrieve payment intent: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Error: Failed to create payment intent");
            handle_error(e);
        }
    }

    println!("\n3. Example completed!");

    Ok(())
}

fn handle_error(error: Error) {
    match error {
        Error::Api {
            kind,
            message,
            status_code,
            request_id,
        } => {
            println!("  Error Type: {:?}", kind);
            println!("  Message: {}", message);
            if let Some(code) = status_code {
                println!("  Status Code: {}", code);
            }
            if let Some(id) = request_id {
                println!("  Request ID: {}", id);
            }

            match kind {
                ErrorKind::Authentication => {
                    println!("\n  Tip: Check your API key is correct");
                }
                ErrorKind::RateLimit => {
                    println!(
                        "\n  Tip: You've exceeded the rate limit. Please wait before retrying."
                    );
                }
                ErrorKind::InvalidRequest => {
                    println!("\n  Tip: Check your request parameters");
                }
                _ => {}
            }
        }
        Error::Timeout(duration) => {
            println!("  Request timed out after {:?}", duration);
            println!("\n  Tip: Check your network connection or increase the timeout");
        }
        Error::InvalidApiKey(msg) => {
            println!("  Invalid API key: {}", msg);
            println!("\n  Tip: Set the PAYREX_API_KEY environment variable");
        }
        _ => {
            println!("  Error: {}", error);
        }
    }
}
