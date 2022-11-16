use crate::redis::client::RedisClient;
use crate::user::api::payloads::{LoginPayload, RefreshPayload, RegistrationPayload};
use crate::user::api::responses::{LoginResponse, RefreshResponse, RegistrationResponse};
use crate::user::auth;
use crate::user::session::models::SessionClaims;
use crate::utils::api::types::Response;
use crate::utils::types::ServiceResult;
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
) -> ServiceResult<HttpResponse> {
    let user = auth::register_user(payload.into_inner(), redis.get_ref().clone()).await?;

    let response = RegistrationResponse {
        username: user.username,
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
    let tokens = auth::login_user(payload.into_inner(), redis.get_ref().clone()).await?;

    let response = LoginResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
    };

    Ok(
        Response::<LoginResponse>::new(StatusCode::OK, "Logged in successfully")
            .data(response)
            .into(),
    )
}

pub async fn handle_logout(
    claims: web::ReqData<SessionClaims>,
    redis: web::Data<Arc<RedisClient>>,
) -> ServiceResult<HttpResponse> {
    auth::logout_user(claims.into_inner(), redis.get_ref().clone()).await?;

    Ok(Response::<()>::new(StatusCode::OK, "Logged out successfully").into())
}

pub async fn handle_refresh(
    redis: web::Data<Arc<RedisClient>>,
    payload: web::Json<RefreshPayload>,
) -> ServiceResult<HttpResponse> {
    let tokens = auth::refresh_user(payload.into_inner(), redis.get_ref().clone()).await?;

    let response = RefreshResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
    };

    Ok(
        Response::<RefreshResponse>::new(StatusCode::OK, "Refreshed successfully")
            .data(response)
            .into(),
    )
}
