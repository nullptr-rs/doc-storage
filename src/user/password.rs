use crate::utils::api::errors::ServiceError;
use crate::utils::types::ServiceResult;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

pub fn hash_password(password: &str) -> ServiceResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| ServiceError::InternalServerError("Failed to hash password".to_string()))?;

    Ok(password_hash.to_string())
}

pub fn verify_password(password: String, password_hash: String) -> ServiceResult<bool> {
    let argon2 = Argon2::default();

    let password_hash = PasswordHash::new(&password_hash).map_err(|_| {
        ServiceError::InternalServerError("Failed to compare passwords".to_string())
    })?;
    let is_valid = argon2
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok();

    Ok(is_valid)
}
