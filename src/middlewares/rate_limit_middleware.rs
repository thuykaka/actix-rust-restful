use crate::models::errors::{Error, ErrorToHttp};
use crate::utils::common::get_client_ip;
use actix_web_ratelimit::{RateLimit, config::RateLimitConfig, store::MemoryStore};
use once_cell::sync::Lazy;
use std::sync::Arc;

use crate::config;

static STORE: Lazy<Arc<MemoryStore>> = Lazy::new(|| Arc::new(MemoryStore::new()));

static CONFIG: Lazy<RateLimitConfig> = Lazy::new(|| {
    // Configure rate limiting: allow 3 requests per 10-second window
    RateLimitConfig::default()
        .max_requests(*config::RATE_LIMIT_MAX_REQUESTS)
        .window_secs(*config::RATE_LIMIT_WINDOW_SECS)
        .id(|req| get_client_ip(req))
        // exceeded(|id, config, _req| {})
        .exceeded(|_, _, _| Error::TooManyRequests.to_http_response())
});

pub fn rate_limiter_middleware() -> RateLimit<Arc<MemoryStore>> {
    RateLimit::new(CONFIG.clone(), STORE.clone())
}
