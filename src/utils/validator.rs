use once_cell::sync::Lazy;
use regex::Regex;
use validator::ValidationError;

static RE_SPECIAL_CHAR: Lazy<Regex> = Lazy::new(|| Regex::new(r"^.*?[@$!%*?&].*$").unwrap());

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    let mut has_whitespace = false;
    let mut has_upper = false;
    let mut has_lower = false;
    let mut has_digit = false;

    for c in password.chars() {
        has_whitespace |= c.is_whitespace();
        has_lower |= c.is_lowercase();
        has_upper |= c.is_uppercase();
        has_digit |= c.is_digit(10);
    }
    if !has_whitespace
        && has_upper
        && has_lower
        && has_digit
        && password.len() >= 8
        && RE_SPECIAL_CHAR.is_match(password)
    {
        Ok(())
    } else {
        Err(ValidationError::new(
            "Password must contain at least one upper case, lower case, number, special character, and must be at least 8 characters long. Dont use spaces.",
        ))
    }
}
