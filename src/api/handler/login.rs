use crate::api::utils::errors::ServiceError;
use crate::api::utils::payloads::{LoginPayload, RegistrationPayload};
use crate::api::utils::responses::{LoginResponse, RegistrationResponse};
use crate::api::utils::types::Response;
use crate::conditional;
use crate::redis::{RedisClient, RedisKey};
use crate::user::User;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse};
use std::sync::Arc;
use uuid::Uuid;

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

    let user = User::new(&payload.username, &payload.password, &payload.device_id).map_err(|error| {
        ServiceError::InternalServerError(
            "Failed to create a new user".to_string(),
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

    conditional!(!exists, {
        return Err(ServiceError::BadRequest(
            "An account with that username does not exist.".to_string(),
        ));
    });

    let user = redis
        .d_async_get::<User>(RedisKey::Account(payload.username.clone()))
        .await?;
    let valid = User::verify_password(payload.password.clone(), user.password.clone()).map_err(
        |error| {
            ServiceError::InternalServerError(
                "Failed to verify the password".to_string(),
                Some(error.into()),
            )
        },
    )?;

    conditional!(!valid, {
        return Err(ServiceError::BadRequest("Invalid password.".to_string()));
    });

    let response = LoginResponse {
        token: Uuid::new_v4().to_string(),
    };

    Ok(
        Response::<LoginResponse>::new(StatusCode::OK, "Logged in successfully")
            .data(response)
            .into(),
    )
}

pub async fn handle_logout() -> Result<HttpResponse, ServiceError> {
    Ok(Response::<()>::new(StatusCode::OK, "Logged out successfully").into())
}
