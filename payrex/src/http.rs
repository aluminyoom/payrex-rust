//! HTTP client implementation with retry logic and error handling.
//!
//! This module provides a wrapper around `reqwest` with automatic retries,
//! rate limiting, and proper error handling for the PayRex API.

use crate::{Config, Error, ErrorKind, Result};
use base64::{Engine as _, engine::general_purpose};
use reqwest::{Client as ReqwestClient, RequestBuilder, Response, StatusCode, header};
use serde::{Serialize, de::DeserializeOwned};
use std::time::Duration;

/// HTTP client for making requests to the PayRex API.
pub(crate) struct HttpClient {
    client: ReqwestClient,
    config: Config,
}

impl HttpClient {
    pub fn new(config: Config) -> Result<Self> {
        let mut headers = header::HeaderMap::new();

        let credentials = format!("{}:", config.api_key());
        let encoded = general_purpose::STANDARD.encode(credentials.as_bytes());
        let auth_value = format!("Basic {encoded}");
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&auth_value)
                .map_err(|e| Error::Config(format!("Invalid API key format: {e}")))?,
        );

        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str(config.user_agent())
                .map_err(|e| Error::Config(format!("Invalid user agent: {e}")))?,
        );

        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        let client = ReqwestClient::builder()
            .default_headers(headers)
            .timeout(config.timeout())
            .build()
            .map_err(|e| Error::Config(format!("Failed to build HTTP client: {e}")))?;

        Ok(Self { client, config })
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = self.build_url(path)?;
        self.execute_with_retry(|| self.client.get(&url)).await
    }

    pub async fn get_with_params<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = self.build_url(path)?;
        let form_data = serde_qs::to_string(body)
            .map_err(|e| Error::Config(format!("Failed to serialize request body: {e}")))?;
        self.execute_with_retry(|| self.client.get(&url).body(form_data.clone()))
            .await
    }

    pub async fn post<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T> {
        let url = self.build_url(path)?;
        let form_data = serde_qs::to_string(body)
            .map_err(|e| Error::Config(format!("Failed to serialize request body: {e}")))?;
        self.execute_with_retry(|| self.client.post(&url).body(form_data.clone()))
            .await
    }

    #[allow(dead_code)]
    pub async fn put<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T> {
        let url = self.build_url(path)?;
        let form_data = serde_qs::to_string(body)
            .map_err(|e| Error::Config(format!("Failed to serialize request body: {e}")))?;
        self.execute_with_retry(|| self.client.put(&url).body(form_data.clone()))
            .await
    }

    pub async fn patch<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = self.build_url(path)?;
        let form_data = serde_qs::to_string(body)
            .map_err(|e| Error::Config(format!("Failed to serialize request body: {e}")))?;
        self.execute_with_retry(|| self.client.patch(&url).body(form_data.clone()))
            .await
    }

    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = self.build_url(path)?;
        self.execute_with_retry(|| self.client.delete(&url)).await
    }

    fn build_url(&self, path: &str) -> Result<String> {
        let base = self.config.api_base_url().trim_end_matches('/');
        let path = path.trim_start_matches('/');
        Ok(format!("{base}/{path}"))
    }

    async fn execute_with_retry<F, T>(&self, request_builder: F) -> Result<T>
    where
        F: Fn() -> RequestBuilder,
        T: DeserializeOwned,
    {
        let mut attempts = 0;
        let max_retries = self.config.max_retries();

        loop {
            let request = request_builder();

            match self.execute_request(request).await {
                Ok(response) => return self.handle_response(response).await,
                Err(e) if e.is_retryable() && attempts < max_retries => {
                    attempts += 1;
                    let delay = self.calculate_retry_delay(attempts);
                    tokio::time::sleep(delay).await;
                }
                Err(e) => return Err(e),
            }
        }
    }

    async fn execute_request(&self, request: RequestBuilder) -> Result<Response> {
        request.send().await.map_err(|e| {
            if e.is_timeout() {
                Error::Timeout(self.config.timeout())
            } else {
                Error::Http(e)
            }
        })
    }

    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        let status = response.status();
        let request_id = response
            .headers()
            .get("x-request-id")
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        if status == StatusCode::TOO_MANY_REQUESTS {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok())
                .map(Duration::from_secs);

            return Err(Error::RateLimit { retry_after });
        }

        if !status.is_success() {
            let error_body = response.text().await.unwrap_or_default();
            let kind = Self::status_to_error_kind(status);

            return Err(Error::Api {
                kind,
                message: error_body,
                status_code: Some(status.as_u16()),
                request_id,
            });
        }

        response.json().await.map_err(Error::Http)
    }

    fn status_to_error_kind(status: StatusCode) -> ErrorKind {
        match status {
            StatusCode::BAD_REQUEST => ErrorKind::InvalidRequest,
            StatusCode::UNAUTHORIZED => ErrorKind::Authentication,
            StatusCode::FORBIDDEN => ErrorKind::PermissionDenied,
            StatusCode::NOT_FOUND => ErrorKind::NotFound,
            StatusCode::TOO_MANY_REQUESTS => ErrorKind::RateLimit,
            s if s.is_server_error() => ErrorKind::ServerError,
            _ => ErrorKind::Unknown,
        }
    }

    fn calculate_retry_delay(&self, attempt: u32) -> Duration {
        let base_delay = self.config.retry_delay();
        let multiplier = 2_u32.pow(attempt.saturating_sub(1));
        base_delay * multiplier
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_url() {
        let config = Config::new("test_key").unwrap();
        let client = HttpClient::new(config).unwrap();

        let url = client.build_url("/payment_intents").unwrap();
        assert!(url.contains("payment_intents"));
        assert!(url.starts_with("https://"));
    }

    #[test]
    fn test_calculate_retry_delay() {
        let config = Config::builder()
            .api_key("test_key")
            .retry_delay(Duration::from_millis(100))
            .build()
            .unwrap();
        let client = HttpClient::new(config).unwrap();

        assert_eq!(client.calculate_retry_delay(1), Duration::from_millis(100));
        assert_eq!(client.calculate_retry_delay(2), Duration::from_millis(200));
        assert_eq!(client.calculate_retry_delay(3), Duration::from_millis(400));
    }

    #[test]
    fn test_status_to_error_kind() {
        assert_eq!(
            HttpClient::status_to_error_kind(StatusCode::BAD_REQUEST),
            ErrorKind::InvalidRequest
        );
        assert_eq!(
            HttpClient::status_to_error_kind(StatusCode::UNAUTHORIZED),
            ErrorKind::Authentication
        );
        assert_eq!(
            HttpClient::status_to_error_kind(StatusCode::NOT_FOUND),
            ErrorKind::NotFound
        );
    }
}
