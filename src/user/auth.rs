use crate::conditional;
use crate::redis::client::RedisClient;
use crate::user::api::payloads::{LoginPayload, RefreshPayload, RegistrationPayload};
use crate::user::models::User;
use crate::user::session::models::{AuthenticationTokens, SessionClaims, SessionRefreshClaims};
use crate::user::session::token;
use crate::utils::api::errors::ServiceError;
use crate::utils::traits::RedisStorable;
use crate::utils::types::ServiceResult;
use std::sync::Arc;

pub async fn register_user(
    payload: RegistrationPayload,
    redis: Arc<RedisClient>,
) -> ServiceResult<User> {
    let exists = User::exists_async(payload.username.clone(), redis.clone()).await?;
    conditional!(
        exists,
        return Err("An account with that username already exists."
            .to_string()
            .into())
    );

    let user = User::new_hashed(payload.username, payload.password, payload.device_id)?;
    user.save_async(redis).await?;

    Ok(user)
}

pub async fn login_user(
    payload: LoginPayload,
    redis: Arc<RedisClient>,
) -> ServiceResult<AuthenticationTokens> {
    let exists = User::exists_async(payload.username.clone(), redis.clone()).await?;
    conditional!(
        exists,
        return Err("An account with that username does not exist."
            .to_string()
            .into())
    );

    let user = User::fetch_async(payload.username.clone(), redis.clone()).await?;
    let valid = user.verify_password(payload.password.clone())?;
    conditional!(valid, return Err("Invalid password".to_string().into()));

    let tokens = token::create_tokens(payload.username.clone(), payload.device_id.clone())?;
    tokens.save_async(redis.clone()).await?;

    Ok(tokens)
}

pub async fn refresh_user(
    payload: RefreshPayload,
    redis: Arc<RedisClient>,
) -> ServiceResult<AuthenticationTokens> {
    let claims = token::decode_refresh_token(payload.refresh_token.clone())?;

    let exists = SessionRefreshClaims::exists_async(claims.jti.clone(), redis.clone()).await?;
    conditional!(!exists, return Err(ServiceError::InvalidToken));

    let access_exists =
        SessionClaims::exists_async(claims.access_token_jti.clone(), redis.clone()).await?;
    conditional!(!access_exists, return Err(ServiceError::InvalidToken));

    let session_claims =
        SessionRefreshClaims::fetch_async(claims.access_token_jti.clone(), redis.clone()).await?;

    session_claims.delete_async(redis.clone()).await?;
    claims.delete_async(redis.clone()).await?;

    let tokens = token::create_tokens(
        session_claims.username.clone(),
        session_claims.device_id.clone(),
    )?;
    tokens.save_async(redis.clone()).await?;

    Ok(tokens)
}

pub async fn logout_user(claims: SessionClaims, redis: Arc<RedisClient>) -> ServiceResult<()> {
    let exists = SessionClaims::exists_async(claims.jti.clone(), redis.clone()).await?;
    conditional!(!exists, return Err(ServiceError::InvalidToken));

    let refresh_claims = SessionClaims::fetch_async(claims.jti.clone(), redis.clone()).await?;

    claims.delete_async(redis.clone()).await?;
    refresh_claims.delete_async(redis.clone()).await?;

    Ok(())
}
