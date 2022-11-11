use crate::constants::ISSUER;
use crate::jwt::token::TokenType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Claims {
    pub token_type: TokenType,

    pub username: String,
    pub device_id: String,

    pub exp: usize,
    pub iat: usize,
    pub iss: String,
    pub jti: String,
}

impl Claims {
    pub fn new(token_type: TokenType, username: String, device_id: String, jti: String) -> Self {
        let now = chrono::Utc::now().timestamp() as usize;

        Self {
            token_type: token_type.clone(),
            username,
            device_id,
            exp: token_type.get_expiration() + now,
            iat: now,
            iss: ISSUER.to_string(),
            jti,
        }
    }
}
