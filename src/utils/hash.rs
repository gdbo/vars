use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand_core::OsRng;

use crate::errors::{AppResult, Error};

pub fn generate_hash(password: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(Error::HashPassword)?;
    Ok(base64::encode(password_hash.to_string()))
}

pub fn verify_password(password: &str, password_hash: &str) -> AppResult<bool> {
    let decode_hash = base64::decode(password_hash).unwrap();
    let decode_hash_str = String::from_utf8_lossy(&decode_hash);

    let password_hash = PasswordHash::new(&decode_hash_str)?;
    let argon2 = Argon2::default();
    Ok(argon2
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok())
}
