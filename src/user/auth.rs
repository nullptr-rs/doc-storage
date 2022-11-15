use std::sync::Arc;
use crate::api::payloads::{LoginPayload, RefreshPayload, RegistrationPayload};
use crate::api::utils::errors::ServiceError;
use crate::api::utils::types::{AccessToken, AuthTokens, RefreshToken, ServiceResult};
use crate::conditional;
use crate::jwt::models::Claims;
use crate::jwt::token;
use crate::jwt::token::{REFRESH_EXPIRATION_TIME, TokenType};
use crate::redis::client::{RedisClient, RedisKey};
use crate::user::models::User;

pub async fn register_user(payload: RegistrationPayload, redis: Arc<RedisClient>) -> ServiceResult<User> {
    let exists = redis
        .async_exists(RedisKey::Account(payload.username.clone()))
        .await?;

    conditional!(exists, return Err("An account with that username already exists.".to_string()));

    let user = User::new_hashed(
        payload.username,
        payload.password,
        payload.device_id,
    )?;

    redis
        .s_async_set(RedisKey::Account(user.username.clone()), &user)
        .await?;

    Ok(user)
}

pub async fn login_user(payload: LoginPayload, redis: Arc<RedisClient>) -> ServiceResult<AuthTokens> {
    let exists = redis
        .async_exists(RedisKey::Account(payload.username.clone()))
        .await?;

    conditional!(exists, return Err("An account with that username does not exist.".to_string()));

    let user = redis
        .d_async_get::<User>(RedisKey::Account(payload.username.clone()))
        .await?;
    let valid = user.verify_password(payload.password.clone())?;

    conditional!(valid, return Err("Invalid password".to_string()));

    let (access_token, refresh_token) =
        token::create_login_tokens(payload.username.clone(), payload.device_id.clone())?;

    Ok((access_token, refresh_token))
}

pub async fn refresh_user(payload: RefreshPayload, redis: Arc<RedisClient>) -> ServiceResult<AuthTokens> {
    let claims = token::decode_token(&payload.refresh_token, TokenType::RefreshToken)
        .map_err(|_| ServiceError::InvalidToken)?;

    let exists = redis.exists(RedisKey::Session(claims.jti.clone()))?;
    conditional!(!exists, return Err(ServiceError::InvalidToken));

    let (access_token, refresh_token) =
        token::create_login_tokens(claims.username.clone(), claims.device_id.clone())?;

    Ok(access_token)
}

pub async fn logout_user(claims: Claims, redis: Arc<RedisClient>) -> ServiceResult<()> {
    redis
        .async_set(RedisKey::SessionBlackList(claims.jti.clone()), "true")
        .await?;
    redis
        .async_expire(
            RedisKey::SessionBlackList(claims.jti.clone()),
            REFRESH_EXPIRATION_TIME,
        )
        .await?;

    Ok(())
}