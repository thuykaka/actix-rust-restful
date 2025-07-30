use once_cell::sync::Lazy;
use std::env;

pub static PORT: Lazy<u16> = Lazy::new(|| {
    env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a valid number")
});
