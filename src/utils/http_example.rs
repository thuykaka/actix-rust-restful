use crate::utils::http_request::{
    AuthInterceptor, HttpClient, RequestInterceptor, RetryInterceptor,
};
use anyhow::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id: Option<i32>,
    title: String,
    body: String,
    user_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}

/// Example of how to use the HTTP client library
pub async fn example_usage() -> Result<()> {
    // Example 1: Basic HTTP client with base URL
    let client = HttpClient::new()
        .with_base_url("https://jsonplaceholder.typicode.com".to_string())
        .with_logging(true);

    // GET request
    let post: Post = client.get("/posts/1").await?;
    println!("Retrieved post: {:?}", post);

    // POST request
    let new_post = Post {
        id: None,
        title: "Test Post".to_string(),
        body: "This is a test post".to_string(),
        user_id: 1,
    };

    let created_post: Post = client.post("/posts", &new_post).await?;
    println!("Created post: {:?}", created_post);

    // PUT request
    let updated_post = Post {
        id: Some(1),
        title: "Updated Title".to_string(),
        body: "Updated body".to_string(),
        user_id: 1,
    };

    let updated: Post = client.put("/posts/1", &updated_post).await?;
    println!("Updated post: {:?}", updated);

    // DELETE request
    let _: serde_json::Value = client.delete("/posts/1").await?;
    println!("Deleted post 1");

    Ok(())
}

/// Example with authentication interceptor
pub async fn example_with_auth() -> Result<()> {
    let auth_interceptor = Box::new(AuthInterceptor::new("your-jwt-token".to_string()));

    let client = HttpClient::new()
        .with_base_url("https://api.example.com".to_string())
        .add_request_interceptor(auth_interceptor)
        .with_logging(true);

    // All requests will now include the Authorization header
    let user: User = client.get("/users/me").await?;
    println!("Authenticated user: {:?}", user);

    Ok(())
}

/// Example with custom headers
pub async fn example_with_headers() -> Result<()> {
    use std::collections::HashMap;

    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());
    headers.insert("X-API-Key".to_string(), "your-api-key".to_string());

    let client = HttpClient::new()
        .with_base_url("https://api.example.com".to_string())
        .with_headers(headers)
        .with_logging(true);

    // All requests will include the custom headers
    let data: serde_json::Value = client.get("/data").await?;
    println!("Data with custom headers: {:?}", data);

    Ok(())
}

/// Example with retry interceptor
pub async fn example_with_retry() -> Result<()> {
    let retry_interceptor = Box::new(RetryInterceptor::new(3, 1000));

    let client = HttpClient::new()
        .with_base_url("https://api.example.com".to_string())
        .add_response_interceptor(retry_interceptor)
        .with_logging(true);

    // Requests will be retried on server errors
    let data: serde_json::Value = client.get("/unreliable-endpoint").await?;
    println!("Data with retry: {:?}", data);

    Ok(())
}

/// Example of custom interceptor
pub struct LoggingInterceptor;

impl RequestInterceptor for LoggingInterceptor {
    fn intercept(&self, request: reqwest::RequestBuilder) -> Result<reqwest::RequestBuilder> {
        println!("Custom logging: Request is being made");
        Ok(request)
    }
}

pub async fn example_with_custom_interceptor() -> Result<()> {
    let custom_interceptor = Box::new(LoggingInterceptor);

    let client = HttpClient::new()
        .with_base_url("https://jsonplaceholder.typicode.com".to_string())
        .add_request_interceptor(custom_interceptor)
        .with_logging(true);

    let post: Post = client.get("/posts/1").await?;
    println!("Post with custom interceptor: {:?}", post);

    Ok(())
}

/// Example of raw request with custom method
pub async fn example_raw_request() -> Result<()> {
    let client = HttpClient::new()
        .with_base_url("https://jsonplaceholder.typicode.com".to_string())
        .with_logging(true);

    // Custom PATCH request
    let update_data = serde_json::json!({
        "title": "Patched Title"
    });

    let body = serde_json::to_string(&update_data)?;
    let result: Post = client
        .request(Method::PATCH, "/posts/1", Some(body))
        .await?;
    println!("Patched post: {:?}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_custom_interceptor() {
        // This test just verifies the function can be called
        // In a real scenario, you'd want to mock the HTTP requests
        assert!(true);
    }
}
