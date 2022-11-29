use std::fmt::{Debug, Display, Formatter};

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

use crate::api::responses::Response;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceError {
    InternalServerError(String),
    BadRequest(String),
    NotFound(String),
    Unauthorized,
}

impl From<String> for ServiceError {
    fn from(error: String) -> Self {
        ServiceError::BadRequest(error)
    }
}

impl<'a> From<&'a str> for ServiceError {
    fn from(error: &'a str) -> Self {
        ServiceError::BadRequest(error.to_string())
    }
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceError::InternalServerError(message) => {
                write!(f, "Internal server error: {}", message)
            }
            ServiceError::BadRequest(message) => write!(f, "Bad request: {}", message),
            ServiceError::NotFound(message) => write!(f, "Resource not found: {}", message),
            ServiceError::Unauthorized => write!(f, "Unauthorized"),
        }
    }
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ServiceError::NotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError(_) => {
                log::error!("{}", self);
                Response::<()>::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
                    .into()
            }
            ServiceError::BadRequest(message) => {
                let message = format!("Bad request: {}", message);
                Response::<()>::new(StatusCode::BAD_REQUEST, &message).into()
            }
            ServiceError::NotFound(message) => {
                let message = format!("Resource not found: {}", message);
                Response::<()>::new(StatusCode::NOT_FOUND, &message).into()
            }
            ServiceError::Unauthorized => {
                Response::<()>::new(StatusCode::UNAUTHORIZED, "Unauthorized").into()
            }
        }
    }
}
