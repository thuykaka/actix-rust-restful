#![allow(dead_code)]
use anyhow::Result;
use reqwest::{
    Client, Method, RequestBuilder,
    header::{HeaderMap, HeaderValue},
};
use serde::{Serialize, de::DeserializeOwned};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use uuid::Uuid;

// Custom error types for HTTP requests
#[derive(Debug, thiserror::Error)]
pub enum HttpRequestError {
    #[error("HTTP request failed with status {status}: {message}")]
    HttpError { status: u16, message: String },

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Invalid header: {0}")]
    InvalidHeader(String),
}

// Configuration for HTTP requests
#[derive(Clone)]
pub struct HttpRequestConfig {
    base_url: Option<String>,
    timeout: Duration,
    headers: HeaderMap,
    retry_attempts: u32,
    retry_delay: Duration,
}

impl Default for HttpRequestConfig {
    fn default() -> Self {
        Self {
            base_url: None,
            timeout: Duration::from_secs(30),
            headers: HeaderMap::new(),
            retry_attempts: 3,
            retry_delay: Duration::from_millis(100),
        }
    }
}

impl HttpRequestConfig {
    // Create a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    // Set the base URL for all requests
    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = Some(base_url);
        self
    }

    // Set the timeout for requests
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    // Add a header to all requests
    pub fn with_header(mut self, key: &str, value: &str) -> Result<Self, HttpRequestError> {
        let header_name = key
            .parse::<reqwest::header::HeaderName>()
            .map_err(|_| HttpRequestError::InvalidHeader(key.to_string()))?;
        let header_value = HeaderValue::from_str(value)
            .map_err(|_| HttpRequestError::InvalidHeader(value.to_string()))?;

        self.headers.insert(header_name, header_value);
        Ok(self)
    }

    // Set retry configuration
    pub fn with_retry(mut self, attempts: u32, delay: Duration) -> Self {
        self.retry_attempts = attempts;
        self.retry_delay = delay;
        self
    }
}

// Request options for individual HTTP requests
#[derive(Clone)]
pub struct RequestOptions {
    headers: HashMap<String, String>,
    query_params: HashMap<String, String>,
    timeout: Option<Duration>,
}

impl Default for RequestOptions {
    fn default() -> Self {
        Self {
            headers: HashMap::new(),
            query_params: HashMap::new(),
            timeout: None,
        }
    }
}

impl RequestOptions {
    // Add a header for this specific request
    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    // Add a query parameter
    pub fn with_query_param(mut self, key: &str, value: &str) -> Self {
        self.query_params.insert(key.to_string(), value.to_string());
        self
    }

    // Set a custom timeout for this request
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
}

// HTTP request service with enhanced functionality
#[derive(Clone)]
pub struct HttpRequestService {
    client: Client,
    config: HttpRequestConfig,
}

impl HttpRequestService {
    // Create a new HTTP request service with default configuration
    pub fn new() -> Result<Self, HttpRequestError> {
        Self::with_config(HttpRequestConfig::default())
    }

    // Create a new HTTP request service with custom configuration
    pub fn with_config(config: HttpRequestConfig) -> Result<Self, HttpRequestError> {
        let client = Client::builder()
            .timeout(config.timeout)
            .default_headers(config.headers.clone())
            .build()
            .map_err(|e| HttpRequestError::RequestError(reqwest::Error::from(e)))?;

        Ok(Self { client, config })
    }

    // Create a new HTTP request service with a builder pattern
    pub fn builder() -> HttpRequestServiceBuilder {
        HttpRequestServiceBuilder::new()
    }

    // Build the full URL from base URL and path
    fn build_url(&self, url: &str) -> Result<String, HttpRequestError> {
        if let Some(base_url) = &self.config.base_url {
            if url.starts_with("http://") || url.starts_with("https://") {
                Ok(url.to_string())
            } else {
                Ok(format!("{}{}", base_url.trim_end_matches('/'), url))
            }
        } else {
            Ok(url.to_string())
        }
    }

    // Build URL with query parameters
    fn build_url_with_params(
        &self,
        url: &str,
        params: &HashMap<String, String>,
    ) -> Result<String, HttpRequestError> {
        let mut full_url = self.build_url(url)?;

        if !params.is_empty() {
            let query_string: Vec<String> = params
                .iter()
                .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
                .collect();
            full_url.push_str(&format!("?{}", query_string.join("&")));
        }

        Ok(full_url)
    }

    // Create a request builder with proper configuration
    fn create_request(
        &self,
        method: Method,
        url: &str,
        options: &RequestOptions,
    ) -> Result<RequestBuilder, HttpRequestError> {
        let full_url = self.build_url_with_params(url, &options.query_params)?;
        let mut request = self.client.request(method, &full_url);

        // Add request-specific headers
        for (key, value) in &options.headers {
            let header_name = key
                .parse::<reqwest::header::HeaderName>()
                .map_err(|_| HttpRequestError::InvalidHeader(key.clone()))?;
            let header_value = HeaderValue::from_str(value)
                .map_err(|_| HttpRequestError::InvalidHeader(value.clone()))?;
            request = request.header(header_name, header_value);
        }

        Ok(request)
    }

    // Generate curl command for debugging
    fn log_curl_command(&self, request_id: &str, req: &RequestBuilder) {
        if let Some(cloned_req) = req.try_clone() {
            if let Ok(request) = cloned_req.build() {
                let mut curl_cmd = format!("curl -X {} '{}'", request.method(), request.url());

                for (name, value) in request.headers().iter() {
                    if let Ok(value_str) = value.to_str() {
                        curl_cmd.push_str(&format!(" -H '{}: {}'", name, value_str));
                    }
                }

                if let Some(body) = request.body() {
                    if let Some(bytes) = body.as_bytes() {
                        let body_str = String::from_utf8_lossy(bytes);
                        curl_cmd.push_str(&format!(" -d '{}'", body_str));
                    } else {
                        curl_cmd.push_str(" -d '<non-text body>'");
                    }
                }

                log::info!("reqId_{}, {}", request_id, curl_cmd);
            }
        }
    }

    // Execute HTTP request with retry logic
    async fn execute_with_retry<T>(
        &self,
        method: Method,
        url: &str,
        body: Option<String>,
        options: &RequestOptions,
    ) -> Result<T, HttpRequestError>
    where
        T: DeserializeOwned,
    {
        let mut last_error = None;

        for attempt in 1..=self.config.retry_attempts {
            match self
                .execute_single(method.clone(), url, body.clone(), options)
                .await
            {
                Ok(result) => return Ok(result),
                Err(e) => {
                    last_error = Some(e);
                    if attempt < self.config.retry_attempts {
                        tokio::time::sleep(self.config.retry_delay).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| HttpRequestError::HttpError {
            status: 0,
            message: "All retry attempts failed".to_string(),
        }))
    }

    // Execute a single HTTP request
    async fn execute_single<T>(
        &self,
        method: Method,
        url: &str,
        body: Option<String>,
        options: &RequestOptions,
    ) -> Result<T, HttpRequestError>
    where
        T: DeserializeOwned,
    {
        let request_id = Uuid::new_v4().to_string();
        let start_time = Instant::now();

        let mut request = self.create_request(method.clone(), url, options)?;

        if let Some(body_content) = &body {
            request = request.body(body_content.clone());
        }

        self.log_curl_command(&request_id, &request);

        let response = request.send().await?;
        let status = response.status();
        let response_text = response.text().await?;

        if !status.is_success() {
            let duration = start_time.elapsed();
            log::error!(
                "reqId_{}, failed with status {}, message: {}, took {:?}",
                request_id,
                status,
                response_text,
                duration
            );
            return Err(HttpRequestError::HttpError {
                status: status.as_u16(),
                message: response_text,
            });
        }

        // Deserialize response
        let result: T = serde_json::from_str(&response_text)?;

        let duration = start_time.elapsed().as_millis();
        log::info!("reqId_{} completed took {}ms", request_id, duration);

        Ok(result)
    }

    // Perform a GET request
    pub async fn get<T>(&self, url: &str) -> Result<T, HttpRequestError>
    where
        T: DeserializeOwned,
    {
        self.get_with_options::<T>(url, RequestOptions::default())
            .await
    }

    // Perform a GET request with options
    pub async fn get_with_options<T>(
        &self,
        url: &str,
        options: RequestOptions,
    ) -> Result<T, HttpRequestError>
    where
        T: DeserializeOwned,
    {
        self.execute_with_retry(Method::GET, url, None, &options)
            .await
    }

    // Perform a POST request
    pub async fn post<T, U>(&self, url: &str, data: &U) -> Result<T, HttpRequestError>
    where
        T: DeserializeOwned,
        U: Serialize,
    {
        self.post_with_options::<T, U>(url, data, RequestOptions::default())
            .await
    }

    // Perform a POST request with options
    pub async fn post_with_options<T, U>(
        &self,
        url: &str,
        data: &U,
        options: RequestOptions,
    ) -> Result<T, HttpRequestError>
    where
        T: DeserializeOwned,
        U: Serialize,
    {
        let body = serde_json::to_string(data)?;
        self.execute_with_retry(Method::POST, url, Some(body), &options)
            .await
    }

    // Perform a PUT request
    pub async fn put<T, U>(&self, url: &str, data: &U) -> Result<T, HttpRequestError>
    where
        T: DeserializeOwned,
        U: Serialize,
    {
        self.put_with_options::<T, U>(url, data, RequestOptions::default())
            .await
    }

    // Perform a PUT request with options
    pub async fn put_with_options<T, U>(
        &self,
        url: &str,
        data: &U,
        options: RequestOptions,
    ) -> Result<T, HttpRequestError>
    where
        T: DeserializeOwned,
        U: Serialize,
    {
        let body = serde_json::to_string(data)?;
        self.execute_with_retry(Method::PUT, url, Some(body), &options)
            .await
    }

    // Perform a DELETE request
    pub async fn delete<T>(&self, url: &str) -> Result<T, HttpRequestError>
    where
        T: DeserializeOwned,
    {
        self.delete_with_options::<T>(url, RequestOptions::default())
            .await
    }

    // Perform a DELETE request with options
    pub async fn delete_with_options<T>(
        &self,
        url: &str,
        options: RequestOptions,
    ) -> Result<T, HttpRequestError>
    where
        T: DeserializeOwned,
    {
        self.execute_with_retry(Method::DELETE, url, None, &options)
            .await
    }

    // Perform a custom request
    pub async fn request<T>(
        &self,
        method: Method,
        url: &str,
        body: Option<String>,
    ) -> Result<T, HttpRequestError>
    where
        T: DeserializeOwned,
    {
        self.request_with_options::<T>(method, url, body, RequestOptions::default())
            .await
    }

    // Perform a custom request with options
    pub async fn request_with_options<T>(
        &self,
        method: Method,
        url: &str,
        body: Option<String>,
        options: RequestOptions,
    ) -> Result<T, HttpRequestError>
    where
        T: DeserializeOwned,
    {
        self.execute_with_retry(method, url, body, &options).await
    }
}

// Builder for HttpRequestService
pub struct HttpRequestServiceBuilder {
    config: HttpRequestConfig,
}

impl HttpRequestServiceBuilder {
    pub fn new() -> Self {
        Self {
            config: HttpRequestConfig::default(),
        }
    }

    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.config.base_url = Some(base_url);
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Result<Self, HttpRequestError> {
        self.config = self.config.with_header(key, value)?;
        Ok(self)
    }

    pub fn with_retry(mut self, attempts: u32, delay: Duration) -> Self {
        self.config = self.config.with_retry(attempts, delay);
        self
    }

    pub fn build(self) -> Result<HttpRequestService, HttpRequestError> {
        HttpRequestService::with_config(self.config)
    }
}

impl Default for HttpRequestServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct TestResponse {
        message: String,
    }

    #[test]
    fn test_http_request_service_creation() {
        let service = HttpRequestService::new();
        assert!(service.is_ok());
    }

    #[test]
    fn test_builder_pattern() {
        let service = HttpRequestService::builder()
            .with_base_url("https://httpbin.org".to_string())
            .with_timeout(Duration::from_secs(10))
            .with_retry(2, Duration::from_millis(50))
            .build();

        assert!(service.is_ok());
    }

    #[test]
    fn test_request_options() {
        let options = RequestOptions::default()
            .with_header("Content-Type", "application/json")
            .with_query_param("key", "value");

        assert_eq!(
            options.headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );
        assert_eq!(options.query_params.get("key"), Some(&"value".to_string()));
    }
}
