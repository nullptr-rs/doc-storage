use crate::constants::{
    DECODING_KEY, ENCODING_KEY, HEADER, REFRESH_DECODING_KEY, REFRESH_ENCODING_KEY, VALIDATION,
};
use crate::user::session::models::{AuthenticationTokens, SessionClaims, SessionRefreshClaims};
use crate::utils::api::errors::ServiceError;
use crate::utils::types::ServiceResult;
use jsonwebtoken::errors::ErrorKind;

pub fn create_tokens(username: String, device_id: String) -> ServiceResult<AuthenticationTokens> {
    let access_token = create_access_token(username, device_id)?;
    let refresh_token = create_refresh_token(access_token.1.jti.clone())?;

    Ok(AuthenticationTokens::new(
        access_token.0,
        refresh_token.0,
        access_token.1,
        refresh_token.1,
    ))
}

pub fn create_access_token(
    username: String,
    device_id: String,
) -> ServiceResult<(String, SessionClaims)> {
    let claims = SessionClaims::new(username, device_id);

    let encoded = jsonwebtoken::encode(&HEADER, &claims, &ENCODING_KEY).map_err(|_| {
        ServiceError::InternalServerError("Failed to create access token".to_string())
    })?;

    Ok((encoded, claims))
}

pub fn create_refresh_token(jti: String) -> ServiceResult<(String, SessionRefreshClaims)> {
    let claims = SessionRefreshClaims::new(jti);

    let result = jsonwebtoken::encode(&HEADER, &claims, &REFRESH_ENCODING_KEY).map_err(|_| {
        ServiceError::InternalServerError("Failed to create refresh token".to_string())
    })?;

    Ok((result, claims))
}

pub fn decode_access_token(token: String) -> ServiceResult<SessionClaims> {
    let result = jsonwebtoken::decode::<SessionClaims>(&token, &DECODING_KEY, &VALIDATION)
        .map(|data| data.claims);

    result.map_err(|error| {
        let error = error.into_kind();

        match error {
            ErrorKind::ExpiredSignature => ServiceError::ExpiredToken,
            _ => ServiceError::InvalidToken,
        }
    })
}

pub fn decode_refresh_token(token: String) -> ServiceResult<SessionRefreshClaims> {
    let result =
        jsonwebtoken::decode::<SessionRefreshClaims>(&token, &REFRESH_DECODING_KEY, &VALIDATION)
            .map(|data| data.claims);

    result.map_err(|error| {
        let error = error.into_kind();

        match error {
            ErrorKind::ExpiredSignature => ServiceError::ExpiredToken,
            _ => ServiceError::InvalidToken,
        }
    })
}
