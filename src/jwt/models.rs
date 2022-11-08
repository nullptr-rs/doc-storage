use crate::constants::{EXPIRATION_TIME, ISSUER};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub device_id: String,

    exp: usize,
    iat: usize,
    iss: String,
}

impl Claims {
    pub fn new(username: String, device_id: String) -> Self {
        let now = chrono::Utc::now().timestamp() as usize;

        Self {
            username,
            device_id,
            exp: EXPIRATION_TIME + now,
            iat: now,
            iss: ISSUER.to_string(),
        }
    }
}
