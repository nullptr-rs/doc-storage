use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::user::User;
use crate::utils::constants;


#[derive(Serialize, Deserialize)]
pub struct UserClaims {
    pub id: Uuid,
    pub username: String,
    pub device_id: String,

    exp: usize,
    iat: usize,
    iss: String,
}

impl UserClaims {
    pub fn new(user: &User, device_id: &str) -> Self {
        let now = chrono::Utc::now().timestamp() as usize;

        Self {
            id: user.id,
            username: user.username.clone(),
            device_id: device_id.to_string(),
            exp: constants::EXPIRATION_TIME + now,
            iat: now,
            iss: constants::ISSUER.to_string(),
        }
    }

    pub fn generate_token(&self) -> Result<String, anyhow::Error> {
        jsonwebtoken::encode(&constants::HEADER, self, &constants::ENCODING_KEY).map_err(Into::into)
    }

    pub fn decode_token(token: &str) -> Result<Self, anyhow::Error> {
        jsonwebtoken::decode::<Self>(token, &constants::DECODING_KEY, &constants::VALIDATION)
            .map(|data| data.claims)
            .map_err(Into::into)
    }
}