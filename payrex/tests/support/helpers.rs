use payrex::{Config, ConfigBuilder, Error};
use serde_json::Value;

pub const TEST_API_KEY: &str = "your_api_key";

pub fn mock_config(api_base_url: impl AsRef<str>) -> Result<Config, Error> {
    ConfigBuilder::new()
        .api_key(TEST_API_KEY)
        .api_base_url(api_base_url.as_ref())
        .test_mode(true)
        .build()
}

pub fn create_json_fixture(file_content: &str) -> Value {
    let json_body: Value = serde_json::from_str(file_content)
        .expect("File content must be encoded as UTF-8 and must follow the JSON format.");
    json_body
}
