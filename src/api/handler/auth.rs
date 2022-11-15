use crate::api::payloads::{LoginPayload, RefreshPayload, RegistrationPayload};
use crate::api::responses::{LoginResponse, RefreshResponse, RegistrationResponse};
use crate::api::utils::errors::ServiceError;
use crate::api::utils::types::{Response, ServiceResult};
use crate::{conditional_return, user};
use crate::constants::REFRESH_EXPIRATION_TIME;
use crate::jwt::models::Claims;
use crate::jwt::token;
use crate::jwt::token::TokenType;
use crate::redis::client::{RedisClient, RedisKey};
use crate::user::models::User;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, Scope};
use std::sync::Arc;
use crate::user::auth;

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
    let user = user::auth::register_user(payload.into_inner(), redis.get_ref().clone()).await?;

    let response = RegistrationResponse {
        username: user.username
    };

    Ok(
        Response::<RegistrationResponse>::new(StatusCode::OK, "Account created successfully")
            .data(response)
            .into()
    )
}

pub async fn handle_login(
    payload: web::Json<LoginPayload>,
    redis: web::Data<Arc<RedisClient>>,
) -> ServiceResult<HttpResponse> {
    let (access_token, refresh_token) = user::auth::login_user(payload.into_inner(), redis.get_ref().clone()).await?;

    let response = LoginResponse::new(access_token, refresh_token);

    Ok(
        Response::<LoginResponse>::new(StatusCode::OK, "Logged in successfully")
            .data(response)
            .into(),
    )
}

pub async fn handle_logout(
    claims: web::ReqData<Claims>,
    redis: web::Data<Arc<RedisClient>>,
) -> ServiceResult<HttpResponse> {
    auth::logout_user(claims.into_inner(), redis.get_ref().clone()).await?;

    Ok(Response::<()>::new(StatusCode::OK, "Logged out successfully").into())
}

pub async fn handle_refresh(
    redis: web::Data<Arc<RedisClient>>,
    payload: web::Json<RefreshPayload>,
) -> ServiceResult<HttpResponse> {
    let claims = token::decode_token(&payload.refresh_token, TokenType::RefreshToken)
        .map_err(|_| ServiceError::InvalidToken)?;
    let exists = redis.exists(RedisKey::SessionBlackList(claims.jti.clone()))?;

    conditional_return!(
        exists,
        Err(ServiceError::BadRequest(
            "Invalid refresh token".to_string()
        ))
    );



    redis.set(RedisKey::SessionBlackList(claims.jti.clone()), "true")?;
    redis.expire(
        RedisKey::SessionBlackList(claims.jti),
        REFRESH_EXPIRATION_TIME as u32,
    )?;

    let response = RefreshResponse::new(access_token, refresh_token);

    Ok(
        Response::<RefreshResponse>::new(StatusCode::OK, "Refreshed successfully")
            .data(response)
            .into(),
    )
}
