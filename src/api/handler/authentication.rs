use crate::api::payloads::{LoginPayload, RefreshPayload, RegistrationPayload};
use crate::api::responses::{LoginResponse, RefreshResponse, RegistrationResponse};
use crate::api::utils::errors::ServiceError;
use crate::api::utils::types::{Response, ServiceResult};
use crate::conditional_return;
use crate::jwt::models::Claims;
use crate::jwt::token;
use crate::jwt::token::TokenType;
use crate::redis::client::{RedisClient, RedisKey};
use crate::user::models::User;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, Scope};
use std::sync::Arc;
use crate::constants::{PASSWORD_COMPARISON_ERROR, REFRESH_EXPIRATION_TIME};

pub fn register_endpoints() -> Scope {
    Scope::new("/auth")
        .service(web::resource("/register").route(web::post().to(handle_registration)))
        .service(web::resource("/login").route(web::post().to(handle_login)))
        .service(web::resource("/logout").route(web::post().to(handle_logout)))
        .service(web::resource("/refresh").route(web::post().to(handle_refresh)))
}

pub async fn handle_registration(
    payload: web::Json<RegistrationPayload>,
    redis: web::Data<Arc<RedisClient>>,
) -> ServiceResult<HttpResponse> {
    let exists = redis
        .async_exists(RedisKey::Account(payload.username.clone()))
        .await?;

    conditional_return!(
        exists,
        Err(ServiceError::BadRequest(
            "An account with that username already exists.".to_string(),
        ))
    );

    let mut user = User::new(
        payload.username.clone(),
        payload.password.clone(),
        payload.device_id.clone(),
    );
    user.hash_password().map_err(|error| {
        ServiceError::InternalServerError(
            "Failed to hash password for user".to_string(),
        )
    })?;

    redis
        .s_async_set(RedisKey::Account(payload.username.clone()), &user)
        .await?;

    let response = RegistrationResponse {
        username: payload.username.clone(),
    };

    Ok(
        Response::<RegistrationResponse>::new(StatusCode::OK, "Account created successfully")
            .data(response)
            .into(),
    )
}

pub async fn handle_login(
    payload: web::Json<LoginPayload>,
    redis: web::Data<Arc<RedisClient>>,
) -> ServiceResult<HttpResponse> {
    let exists = redis
        .async_exists(RedisKey::Account(payload.username.clone()))
        .await?;

    conditional_return!(
        !exists,
        Err(ServiceError::BadRequest(
            "An account with that username does not exist.".to_string(),
        ))
    );

    let user = redis
        .d_async_get::<User>(RedisKey::Account(payload.username.clone()))
        .await?;
    let valid = user
        .verify_password(payload.password.clone())
        .map_err(|_| PASSWORD_COMPARISON_ERROR)?;

    conditional_return!(
        !valid,
        Err(ServiceError::BadRequest("Invalid password".to_string()))
    );

    let (access_token, refresh_token) =
        token::create_login_tokens(payload.username.clone(), payload.device_id.clone())?;

    let response = LoginResponse::new(access_token, refresh_token);

    Ok(
        Response::<LoginResponse>::new(StatusCode::OK, "Logged in successfully")
            .data(response)
            .into(),
    )
}

pub async fn handle_logout(
    redis: web::Data<Arc<RedisClient>>,
    claims: web::ReqData<Claims>,
) -> ServiceResult<HttpResponse> {
    redis.set(RedisKey::SessionBlackList(claims.jti.clone()), "true")?;
    redis.expire(RedisKey::SessionBlackList(claims.jti.clone()), REFRESH_EXPIRATION_TIME as u32)?;
    Ok(Response::<()>::new(StatusCode::OK, "Logged out successfully").into())
}

pub async fn handle_refresh(
    redis: web::Data<Arc<RedisClient>>,
    payload: web::Json<RefreshPayload>,
) -> ServiceResult<HttpResponse> {
    let claims = token::decode_token(&payload.refresh_token, TokenType::RefreshToken).map_err(|_| ServiceError::InvalidToken)?;
    let exists = redis.exists(RedisKey::SessionBlackList(claims.jti.clone()))?;

    conditional_return!(
        exists,
        Err(ServiceError::BadRequest(
            "Invalid refresh token".to_string()
        ))
    );

    let (access_token, refresh_token) =
        token::create_login_tokens(claims.username.clone(), claims.device_id.clone())?;

    redis.set(RedisKey::SessionBlackList(claims.jti.clone()), "true")?;
    redis.expire(RedisKey::SessionBlackList(claims.jti), REFRESH_EXPIRATION_TIME as u32)?;

    let response = RefreshResponse::new(access_token, refresh_token);

    Ok(
        Response::<RefreshResponse>::new(StatusCode::OK, "Refreshed successfully")
            .data(response)
            .into(),
    )
}
