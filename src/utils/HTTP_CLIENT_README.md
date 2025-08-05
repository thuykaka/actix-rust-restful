# HTTP Client Library (Axios-like for Rust)

A comprehensive HTTP client library for Rust that provides an axios-like interface with advanced features including interceptors, base URL support, and detailed logging.

## Features

- ✅ **HTTP Methods**: GET, POST, PUT, DELETE, PATCH
- ✅ **Serialization/Deserialization**: Full serde support
- ✅ **Interceptors**: Request and response interceptors
- ✅ **Base URL**: Configure base URL for all requests
- ✅ **Logging**: Detailed request/response logging with curl format
- ✅ **Error Handling**: Comprehensive error handling with anyhow
- ✅ **Async/Await**: Full async support with tokio
- ✅ **Custom Headers**: Support for custom headers
- ✅ **Timeout**: Configurable request timeouts

## Quick Start

```rust
use crate::utils::http_request::HttpClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id: Option<i32>,
    title: String,
    body: String,
    user_id: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create HTTP client with base URL
    let client = HttpClient::new()
        .with_base_url("https://jsonplaceholder.typicode.com".to_string())
        .with_logging(true);

    // GET request
    let post: Post = client.get("/posts/1").await?;
    println!("Post: {:?}", post);

    // POST request
    let new_post = Post {
        id: None,
        title: "Test Post".to_string(),
        body: "This is a test".to_string(),
        user_id: 1,
    };

    let created: Post = client.post("/posts", &new_post).await?;
    println!("Created: {:?}", created);

    Ok(())
}
```

## Configuration

### Basic Configuration

```rust
let client = HttpClient::new()
    .with_base_url("https://api.example.com".to_string())
    .with_logging(true);
```

### Advanced Configuration

```rust
use std::collections::HashMap;
use std::time::Duration;

let mut headers = HashMap::new();
headers.insert("Content-Type".to_string(), "application/json".to_string());
headers.insert("X-API-Key".to_string(), "your-key".to_string());

let config = HttpClientConfig {
    base_url: Some("https://api.example.com".to_string()),
    timeout: Duration::from_secs(30),
    headers,
    request_interceptors: Vec::new(),
    response_interceptors: Vec::new(),
    enable_logging: true,
};

let client = HttpClient::with_config(config);
```

## HTTP Methods

### GET Request

```rust
let data: serde_json::Value = client.get("/users/1").await?;
```

### POST Request

```rust
let user_data = User { name: "John".to_string(), email: "john@example.com".to_string() };
let created: User = client.post("/users", &user_data).await?;
```

### PUT Request

```rust
let updated_data = User { name: "Jane".to_string(), email: "jane@example.com".to_string() };
let updated: User = client.put("/users/1", &updated_data).await?;
```

### DELETE Request

```rust
let _: serde_json::Value = client.delete("/users/1").await?;
```

### PATCH Request

```rust
let patch_data = serde_json::json!({ "name": "Updated Name" });
let body = serde_json::to_string(&patch_data)?;
let updated: User = client.request(Method::PATCH, "/users/1", Some(body)).await?;
```

## Interceptors

### Authentication Interceptor

```rust
use crate::utils::http_request::{AuthInterceptor, RequestInterceptor};

let auth_interceptor = Box::new(AuthInterceptor::new("your-jwt-token".to_string()));

let client = HttpClient::new()
    .add_request_interceptor(auth_interceptor)
    .with_base_url("https://api.example.com".to_string());

// All requests will include Authorization: Bearer your-jwt-token
```

### Custom Request Interceptor

```rust
use reqwest::RequestBuilder;

struct CustomInterceptor;

impl RequestInterceptor for CustomInterceptor {
    fn intercept(&self, request: RequestBuilder) -> Result<RequestBuilder> {
        // Add custom headers, modify request, etc.
        Ok(request.header("X-Custom-Header", "custom-value"))
    }
}

let client = HttpClient::new()
    .add_request_interceptor(Box::new(CustomInterceptor));
```

### Response Interceptor

```rust
use crate::utils::http_request::{ResponseInterceptor, RetryInterceptor};

let retry_interceptor = Box::new(RetryInterceptor::new(3, 1000));

let client = HttpClient::new()
    .add_response_interceptor(retry_interceptor);
```

## Logging

The library provides detailed logging of requests and responses:

### Request Logging

```
INFO HTTP Request (curl): curl -X GET 'https://api.example.com/users/1' -H 'Content-Type: application/json'
```

### Response Logging

```
INFO HTTP Response - Status: 200 OK
INFO HTTP Response - Headers: {"content-type": "application/json"}
INFO HTTP Response - Body: {"id": 1, "name": "John Doe"}
INFO HTTP request completed in 245ms
```

## Error Handling

The library uses `anyhow::Result` for comprehensive error handling:

```rust
match client.get::<User>("/users/1").await {
    Ok(user) => println!("User: {:?}", user),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Examples

See `src/utils/http_example.rs` for comprehensive examples including:

- Basic usage with different HTTP methods
- Authentication with interceptors
- Custom headers configuration
- Retry logic with interceptors
- Custom interceptors
- Raw requests with custom methods

## Testing

Run the tests to see the library in action:

```bash
cargo test --package actix_rust_restful --lib utils::http_request
cargo test --package actix_rust_restful --lib utils::http_example
```

## Dependencies

The library requires these dependencies in `Cargo.toml`:

```toml
[dependencies]
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
tracing = "0.1"
tokio = "1.47.0"
```

## Performance Considerations

- The library uses `reqwest` which is built on top of `hyper` for high performance
- All operations are async and non-blocking
- Connection pooling is handled automatically by `reqwest`
- Timeouts are configurable to prevent hanging requests

## Best Practices

1. **Reuse Clients**: Create one client instance and reuse it for multiple requests
2. **Use Base URLs**: Configure base URLs to avoid repeating full URLs
3. **Handle Errors**: Always handle potential errors from HTTP requests
4. **Use Interceptors**: Implement authentication and retry logic using interceptors
5. **Enable Logging**: Use logging during development for debugging
6. **Set Timeouts**: Configure appropriate timeouts for your use case

## Migration from Axios

If you're familiar with axios in JavaScript, here's how concepts map:

| Axios Concept                 | Rust Equivalent                     |
| ----------------------------- | ----------------------------------- |
| `axios.create({ baseURL })`   | `HttpClient::new().with_base_url()` |
| `axios.interceptors.request`  | `add_request_interceptor()`         |
| `axios.interceptors.response` | `add_response_interceptor()`        |
| `axios.get()`                 | `client.get()`                      |
| `axios.post()`                | `client.post()`                     |
| `axios.put()`                 | `client.put()`                      |
| `axios.delete()`              | `client.delete()`                   |
