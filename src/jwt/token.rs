use crate::api::utils::errors::ServiceError;
use crate::constants::{DECODING_KEY, ENCODING_KEY, HEADER, REFRESH_DECODING_KEY, VALIDATION};
use crate::jwt::models::Claims;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::DecodingKey;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenType {
    AccessToken,
    RefreshToken,
}

impl TokenType {
    pub fn get_decoding_key(&self) -> &DecodingKey {
        match self {
            TokenType::AccessToken => &DECODING_KEY,
            TokenType::RefreshToken => &REFRESH_DECODING_KEY,
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::AccessToken => write!(f, "access_token"),
            TokenType::RefreshToken => write!(f, "refresh_token"),
        }
    }
}

pub fn create_login_tokens(
    username: String,
    device_id: String,
) -> Result<(String, String), ServiceError> {
    let uuid = Uuid::new_v4().to_string();
    let access_token = create_token(
        TokenType::AccessToken,
        username.clone(),
        device_id.clone(),
        uuid.clone(),
    )?;
    let refresh_token = create_token(TokenType::RefreshToken, username, device_id, uuid)?;

    Ok((access_token, refresh_token))
}

pub fn create_token(
    token_type: TokenType,
    username: String,
    device_id: String,
    jti: String,
) -> Result<String, ServiceError> {
    let claims = Claims::new(token_type, username, device_id, jti);

    from_claims(&claims)
}

pub fn from_claims(claims: &Claims) -> Result<String, ServiceError> {
    jsonwebtoken::encode(&HEADER, claims, &ENCODING_KEY).map_err(|error| {
        ServiceError::InternalServerError("Failed to create token".to_string(), Some(error.into()))
    })
}

pub fn decode_token(token: &str, token_type: TokenType) -> Result<Claims, ServiceError> {
    jsonwebtoken::decode::<Claims>(token, token_type.get_decoding_key(), &VALIDATION)
        .map(|data| data.claims)
        .map_err(|error| {
            let error = error.into_kind();

            match error {
                ErrorKind::ExpiredSignature => ServiceError::ExpiredToken,
                _ => ServiceError::InvalidToken,
            }
        })
}
