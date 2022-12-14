use crate::user::password;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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

    pub fn hash_password(&mut self) -> Result<(), anyhow::Error> {
        self.password = password::hash_password(&self.password)?;
        Ok(())
    }

    pub fn verify_password(&self, password: String) -> Result<bool, anyhow::Error> {
        password::verify_password(password, self.password.clone())
    }
}
