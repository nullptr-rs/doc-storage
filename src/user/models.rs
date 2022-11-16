use crate::redis::client::RedisKey;
use crate::user::password;
use crate::utils::traits::RedisStorable;
use crate::utils::types::ServiceResult;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub device_id: Vec<String>,
}

impl User {
    pub fn new(username: String, password: String, device_id: String) -> Self {
        Self {
            username,
            password,
            device_id: vec![device_id],
        }
    }

    pub fn new_hashed(
        username: String,
        password: String,
        device_id: String,
    ) -> ServiceResult<Self> {
        let mut user = Self::new(username, password, device_id);
        user.hash_password()?;

        Ok(user)
    }

    pub fn hash_password(&mut self) -> ServiceResult<()> {
        self.password = password::hash_password(&self.password)?;
        Ok(())
    }

    pub fn verify_password(&self, password: String) -> ServiceResult<bool> {
        password::verify_password(password, self.password.clone())
    }
}

#[async_trait]
impl RedisStorable<User> for User {
    fn key(key: String) -> RedisKey {
        RedisKey::User(key)
    }

    fn self_key(&self) -> RedisKey {
        RedisKey::User(self.username.clone())
    }
}
