use crate::api::utils::errors::ServiceError;
use crate::api::utils::types::{AccessToken, RefreshToken, ServiceResult};
use crate::constants::{
    DECODING_KEY, ENCODING_KEY, EXPIRATION_TIME, HEADER, REFRESH_DECODING_KEY,
    REFRESH_ENCODING_KEY, REFRESH_EXPIRATION_TIME, VALIDATION,
};
use crate::jwt::models::Claims;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenType {
    AccessToken,
    RefreshToken,
}

impl TokenType {
    pub fn get_encoding_key(&self) -> &EncodingKey {
        match self {
            TokenType::AccessToken => &ENCODING_KEY,
            TokenType::RefreshToken => &REFRESH_ENCODING_KEY,
        }
    }

    pub fn get_decoding_key(&self) -> &DecodingKey {
        match self {
            TokenType::AccessToken => &DECODING_KEY,
            TokenType::RefreshToken => &REFRESH_DECODING_KEY,
        }
    }

    pub fn get_expiration(&self) -> usize {
        match self {
            TokenType::AccessToken => EXPIRATION_TIME,
            TokenType::RefreshToken => REFRESH_EXPIRATION_TIME,
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
) -> ServiceResult<(AccessToken, RefreshToken)> {
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
) -> ServiceResult<String> {
    let claims = Claims::new(token_type, username, device_id, jti);

    from_claims(&claims)
}

pub fn from_claims(claims: &Claims) -> ServiceResult<String> {
    let encoding_key = claims.token_type.get_encoding_key();

    jsonwebtoken::encode(&HEADER, claims, encoding_key)
        .map_err(|_| ServiceError::token_generation())
}

pub fn decode_token(token: &str, token_type: TokenType) -> ServiceResult<Claims> {
    let decoding_key = token_type.get_decoding_key();

    let result =
        jsonwebtoken::decode::<Claims>(token, decoding_key, &VALIDATION).map(|data| data.claims);

    result.map_err(|error| {
        let error = error.into_kind();

        match error {
            ErrorKind::ExpiredSignature => ServiceError::ExpiredToken,
            _ => ServiceError::InvalidToken,
        }
    })
}
