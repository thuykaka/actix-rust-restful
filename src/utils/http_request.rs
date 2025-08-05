use anyhow::{Result, anyhow};
use reqwest::{Client, Method, RequestBuilder, Response, StatusCode};
use serde::{Serialize, de::DeserializeOwned};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::Instant;
use tracing::{info, warn};

/// HTTP request interceptor trait
pub trait RequestInterceptor: Send + Sync {
    fn intercept(&self, request: RequestBuilder) -> Result<RequestBuilder>;
}

/// HTTP response interceptor trait
pub trait ResponseInterceptor: Send + Sync {
    fn intercept(&self, response: Response) -> Result<Response>;
}

/// HTTP client configuration
pub struct HttpClientConfig {
    pub base_url: Option<String>,
    pub timeout: Duration,
    pub headers: HashMap<String, String>,
    pub request_interceptors: Vec<Box<dyn RequestInterceptor>>,
    pub response_interceptors: Vec<Box<dyn ResponseInterceptor>>,
    pub enable_logging: bool,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            base_url: None,
            timeout: Duration::from_secs(30),
            headers: HashMap::new(),
            request_interceptors: Vec::new(),
            response_interceptors: Vec::new(),
            enable_logging: true,
        }
    }
}

/// HTTP client similar to axios
pub struct HttpClient {
    client: Client,
    config: HttpClientConfig,
}

impl HttpClient {
    /// Create a new HTTP client with default configuration
    pub fn new() -> Self {
        Self::with_config(HttpClientConfig::default())
    }

    /// Create a new HTTP client with custom configuration
    pub fn with_config(config: HttpClientConfig) -> Self {
        let client = Client::builder()
            .timeout(config.timeout)
            .build()
            .expect("Failed to create HTTP client");

        Self { client, config }
    }

    /// Set base URL for all requests
    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.config.base_url = Some(base_url);
        self
    }

    /// Add request interceptor
    pub fn add_request_interceptor(mut self, interceptor: Box<dyn RequestInterceptor>) -> Self {
        self.config.request_interceptors.push(interceptor);
        self
    }

    /// Add response interceptor
    pub fn add_response_interceptor(mut self, interceptor: Box<dyn ResponseInterceptor>) -> Self {
        self.config.response_interceptors.push(interceptor);
        self
    }

    /// Enable or disable logging
    pub fn with_logging(mut self, enable: bool) -> Self {
        self.config.enable_logging = enable;
        self
    }

    /// Add default headers
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.config.headers = headers;
        self
    }

    /// Build the full URL with base URL if provided
    fn build_url(&self, url: &str) -> String {
        if let Some(base_url) = &self.config.base_url {
            if url.starts_with("http://") || url.starts_with("https://") {
                url.to_string()
            } else {
                format!("{}{}", base_url.trim_end_matches('/'), url)
            }
        } else {
            url.to_string()
        }
    }

    /// Create a request builder with default headers
    fn create_request(&self, method: Method, url: &str) -> RequestBuilder {
        let full_url = self.build_url(url);
        let mut request = self.client.request(method, &full_url);

        // Add default headers
        for (key, value) in &self.config.headers {
            request = request.header(key, value);
        }

        request
    }

    /// Apply request interceptors
    fn apply_request_interceptors(&self, mut request: RequestBuilder) -> Result<RequestBuilder> {
        for interceptor in &self.config.request_interceptors {
            request = interceptor.intercept(request)?;
        }
        Ok(request)
    }

    /// Apply response interceptors
    fn apply_response_interceptors(&self, response: Response) -> Result<Response> {
        let mut final_response = response;
        for interceptor in &self.config.response_interceptors {
            final_response = interceptor.intercept(final_response)?;
        }
        Ok(final_response)
    }

    /// Log request as curl command
    fn log_request(
        &self,
        method: &Method,
        url: &str,
        headers: &HashMap<String, String>,
        body: Option<&str>,
    ) {
        if !self.config.enable_logging {
            return;
        }

        let mut curl_cmd = format!("curl -X {} '{}'", method, url);

        // Add headers
        for (key, value) in headers {
            curl_cmd.push_str(&format!(" -H '{}: {}'", key, value));
        }

        // Add body if present
        if let Some(body) = body {
            curl_cmd.push_str(&format!(" -d '{}'", body));
        }

        info!("HTTP Request (curl): {}", curl_cmd);
    }

    /// Log response
    fn log_response(
        &self,
        status: &StatusCode,
        headers: &HashMap<String, String>,
        body: Option<&str>,
    ) {
        if !self.config.enable_logging {
            return;
        }

        info!("HTTP Response - Status: {}", status);
        info!("HTTP Response - Headers: {:?}", headers);
        if let Some(body) = body {
            info!("HTTP Response - Body: {}", body);
        }
    }

    /// Execute HTTP request with generic response type
    async fn execute<T>(&self, method: Method, url: &str, body: Option<String>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let start_time = Instant::now();
        let mut request = self.create_request(method.clone(), url);

        // Add body if provided
        if let Some(body_content) = &body {
            request = request.body(body_content.clone());
        }

        // Apply request interceptors
        request = self.apply_request_interceptors(request)?;

        // Log request
        self.log_request(&method, url, &self.config.headers, body.as_deref());

        // Execute request
        let response = request.send().await?;
        let status = response.status();
        let response_headers: HashMap<String, String> = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        // Apply response interceptors
        let response = self.apply_response_interceptors(response)?;

        // Get response body
        let response_text = response.text().await?;

        // Log response
        self.log_response(&status, &response_headers, Some(&response_text));

        // Check if request was successful
        if !status.is_success() {
            return Err(anyhow!(
                "HTTP request failed with status {}: {}",
                status,
                response_text
            ));
        }

        // Deserialize response
        let result: T = serde_json::from_str(&response_text)?;

        let duration = start_time.elapsed();
        info!("HTTP request completed in {:?}", duration);

        Ok(result)
    }

    /// GET request
    pub async fn get<T>(&self, url: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.execute(Method::GET, url, None).await
    }

    /// POST request
    pub async fn post<T, U>(&self, url: &str, data: &U) -> Result<T>
    where
        T: DeserializeOwned,
        U: Serialize,
    {
        let body = serde_json::to_string(data)?;
        self.execute(Method::POST, url, Some(body)).await
    }

    /// PUT request
    pub async fn put<T, U>(&self, url: &str, data: &U) -> Result<T>
    where
        T: DeserializeOwned,
        U: Serialize,
    {
        let body = serde_json::to_string(data)?;
        self.execute(Method::PUT, url, Some(body)).await
    }

    /// DELETE request
    pub async fn delete<T>(&self, url: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.execute(Method::DELETE, url, None).await
    }

    /// PATCH request
    pub async fn patch<T, U>(&self, url: &str, data: &U) -> Result<T>
    where
        T: DeserializeOwned,
        U: Serialize,
    {
        let body = serde_json::to_string(data)?;
        self.execute(Method::PATCH, url, Some(body)).await
    }

    /// Raw request with custom method
    pub async fn request<T>(&self, method: Method, url: &str, body: Option<String>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.execute(method, url, body).await
    }
}

/// Authentication interceptor example
pub struct AuthInterceptor {
    token: String,
}

impl AuthInterceptor {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

impl RequestInterceptor for AuthInterceptor {
    fn intercept(&self, request: RequestBuilder) -> Result<RequestBuilder> {
        Ok(request.header("Authorization", format!("Bearer {}", self.token)))
    }
}

/// Retry interceptor example
pub struct RetryInterceptor {
    max_retries: u32,
    delay_ms: u64,
}

impl RetryInterceptor {
    pub fn new(max_retries: u32, delay_ms: u64) -> Self {
        Self {
            max_retries,
            delay_ms,
        }
    }
}

impl ResponseInterceptor for RetryInterceptor {
    fn intercept(&self, response: Response) -> Result<Response> {
        // This is a simplified example - in a real implementation,
        // you would need to handle the retry logic more carefully
        if response.status().is_server_error() {
            warn!("Server error detected, consider implementing retry logic");
        }
        Ok(response)
    }
}

/// Example usage and tests
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn test_with_interceptors() {
        let auth_interceptor = Box::new(AuthInterceptor::new("test-token".to_string()));

        let client = HttpClient::new()
            .add_request_interceptor(auth_interceptor)
            .with_logging(true);

        // This would add Authorization header to all requests
        assert_eq!(client.config.request_interceptors.len(), 1);
    }
}
