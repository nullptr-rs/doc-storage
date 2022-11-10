use crate::api::payloads::{LoginPayload, RefreshPayload, RegistrationPayload};
use crate::api::responses::{LoginResponse, RefreshResponse, RegistrationResponse};
use crate::api::utils::errors::ServiceError;
use crate::api::utils::types::Response;
use crate::conditional_return;
use crate::jwt::models::Claims;
use crate::jwt::token;
use crate::jwt::token::TokenType;
use crate::redis::client::{RedisClient, RedisKey};
use crate::user::models::User;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, Scope};
use std::sync::Arc;

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
) -> Result<HttpResponse, ServiceError> {
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
            "Failed to hash password.".to_string(),
            Some(error.into()),
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
) -> Result<HttpResponse, ServiceError> {
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
        .map_err(|error| {
            ServiceError::InternalServerError(
                "Failed to verify the password".to_string(),
                Some(error.into()),
            )
        })?;

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
) -> Result<HttpResponse, ServiceError> {
    redis.set(RedisKey::SessionBlackList(claims.jti.clone()), "true")?;
    Ok(Response::<()>::new(StatusCode::OK, "Logged out successfully").into())
}

pub async fn handle_refresh(
    redis: web::Data<Arc<RedisClient>>,
    payload: web::Json<RefreshPayload>,
) -> Result<HttpResponse, ServiceError> {
    let claims = token::decode_token(&payload.refresh_token, TokenType::RefreshToken)
        .map_err(|_| ServiceError::InvalidToken)?;
    let exists = redis.exists(RedisKey::SessionBlackList(claims.jti.clone()))?;

    conditional_return!(
        exists,
        Err(ServiceError::BadRequest(
            "Invalid refresh token".to_string()
        ))
    );

    let (access_token, refresh_token) =
        token::create_login_tokens(claims.username.clone(), claims.device_id.clone())?;

    redis.set(RedisKey::SessionBlackList(claims.jti), "true")?;

    let response = RefreshResponse::new(access_token, refresh_token);

    Ok(
        Response::<RefreshResponse>::new(StatusCode::OK, "Refreshed successfully")
            .data(response)
            .into(),
    )
}
