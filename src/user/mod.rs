use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub device_id: Vec<String>,
}

impl User {
    pub fn new(username: &str, password: &str) -> Result<Self, anyhow::Error> {
        let id = Uuid::new_v4();
        let password = Self::hash_password(password)?;

        Ok(Self {
            id,
            username: username.to_string(),
            password,
            device_id: Vec::new(),
        })
    }

    pub fn hash_password(password: &str) -> Result<String, anyhow::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|err| anyhow::anyhow!(err))?;
        Ok(password_hash.to_string())
    }

    pub fn verify_password(password: String, password_hash: String) -> Result<bool, anyhow::Error> {
        let argon2 = Argon2::default();

        let password_hash =
            PasswordHash::new(&password_hash).map_err(|error| anyhow::anyhow!(error))?;
        let is_valid = argon2
            .verify_password(password.as_bytes(), &password_hash)
            .is_ok();

        Ok(is_valid)
    }
}
