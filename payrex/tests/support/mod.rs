mod helpers;

pub use helpers::TEST_API_KEY;
pub use helpers::{create_json_fixture, mock_config};
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
