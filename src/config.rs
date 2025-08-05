use once_cell::sync::Lazy;
use std::env;

pub static PORT: Lazy<u16> = Lazy::new(|| {
    env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a valid number")
});

pub static DATABASE_URL: Lazy<String> = Lazy::new(|| {
    env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/postgres".to_string())
});

pub static JWT_SECRET: Lazy<String> = Lazy::new(|| {
    env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key-change-in-production".to_string())
});

pub static DB_MAX_CONNECTIONS: Lazy<u32> = Lazy::new(|| {
    env::var("DB_MAX_CONNECTIONS")
        .unwrap_or_else(|_| "10".to_string())
        .parse()
        .expect("DB_MAX_CONNECTIONS must be a valid number")
});

pub static RATE_LIMIT_MAX_REQUESTS: Lazy<usize> = Lazy::new(|| {
    env::var("RATE_LIMIT_MAX_REQUESTS")
        .unwrap_or_else(|_| "100".to_string())
        .parse()
        .expect("RATE_LIMIT_MAX_REQUESTS must be a valid number")
});

pub static RATE_LIMIT_WINDOW_SECS: Lazy<u64> = Lazy::new(|| {
    env::var("RATE_LIMIT_WINDOW_SECS")
        .unwrap_or_else(|_| "60".to_string())
        .parse()
        .expect("RATE_LIMIT_WINDOW_SECS must be a valid number")
});

pub static REFRESH_TOKEN_EXPIRATION_HOURS: Lazy<i64> = Lazy::new(|| {
    env::var("REFRESH_TOKEN_EXPIRATION_HOURS")
        .unwrap_or_else(|_| "24".to_string())
        .parse()
        .expect("REFRESH_TOKEN_EXPIRATION_HOURS must be a valid number")
});

pub static ACCESS_TOKEN_EXPIRATION_HOURS: Lazy<i64> = Lazy::new(|| {
    env::var("ACCESS_TOKEN_EXPIRATION_HOURS")
        .unwrap_or_else(|_| "1".to_string())
        .parse()
        .expect("ACCESS_TOKEN_EXPIRATION_HOURS must be a valid number")
});
