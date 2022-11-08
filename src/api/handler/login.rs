use crate::api::utils::errors::ServiceError;
use crate::api::utils::payloads::{LoginPayload, RegistrationPayload};
use crate::api::utils::responses::{LoginResponse, RegistrationResponse};
use crate::api::utils::types::Response;
use crate::conditional;
use crate::jwt::models::Claims;
use crate::jwt::token;
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
}

pub async fn handle_registration(
    payload: web::Json<RegistrationPayload>,
    redis: web::Data<Arc<RedisClient>>,
) -> Result<HttpResponse, ServiceError> {
    let exists = redis
        .async_exists(RedisKey::Account(payload.username.clone()))
        .await?;

    conditional!(exists, {
        return Err(ServiceError::BadRequest(
            "An account with that username already exists.".to_string(),
        ));
    });

    let user = User::new(
        payload.username.clone(),
        payload.password.clone(),
        payload.device_id.clone(),
    );

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

    conditional!(!exists, {
        return Err(ServiceError::BadRequest(
            "An account with that username does not exist.".to_string(),
        ));
    });

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

    conditional!(!valid, {
        return Err(ServiceError::BadRequest("Invalid password.".to_string()));
    });

    let claims = Claims::new(payload.username.clone(), payload.device_id.clone());
    let token = token::from_claims(&claims).map_err(|error| {
        ServiceError::InternalServerError(
            "Failed to generate a token".to_string(),
            Some(error.into()),
        )
    })?;
    let response = LoginResponse { token };

    Ok(
        Response::<LoginResponse>::new(StatusCode::OK, "Logged in successfully")
            .data(response)
            .into(),
    )
}

pub async fn handle_logout(_claims: web::ReqData<Claims>) -> Result<HttpResponse, ServiceError> {
    Ok(Response::<()>::new(StatusCode::OK, "Logged out successfully").into())
}
