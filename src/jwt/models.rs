use crate::constants::{EXPIRATION_TIME, ISSUER, REFRESH_EXPIRATION_TIME};
use crate::jwt::token::TokenType;
use crate::ternary;
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
        let expiration = ternary!(
            token_type == TokenType::AccessToken,
            EXPIRATION_TIME + now,
            REFRESH_EXPIRATION_TIME + now
        );

        Self {
            token_type,
            username,
            device_id,
            exp: expiration,
            iat: now,
            iss: ISSUER.to_string(),
            jti,
        }
    }
}
