use crate::constants::{DECODING_KEY, ENCODING_KEY, HEADER, VALIDATION};
use crate::jwt::models::Claims;

pub fn create_token(username: String, device_id: String) -> Result<String, anyhow::Error> {
    let claims = Claims::new(username, device_id);

    from_claims(&claims)
}

pub fn from_claims(claims: &Claims) -> Result<String, anyhow::Error> {
    jsonwebtoken::encode(&HEADER, claims, &ENCODING_KEY).map_err(Into::into)
}

pub fn decode_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    jsonwebtoken::decode::<Claims>(token, &DECODING_KEY, &VALIDATION).map(|data| data.claims)
}
