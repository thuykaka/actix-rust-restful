use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    password_hash
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let argon2 = Argon2::default();
    let parsed_hash = if let Ok(h) = PasswordHash::new(hash) {
        h
    } else {
        log::error!("verify_password -> failed to parse hash: {}", hash);
        return false;
    };
    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
