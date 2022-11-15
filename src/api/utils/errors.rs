use crate::api::utils::types::Response;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use std::fmt::{Debug, Display, Formatter};

pub enum ServiceError {
    InternalServerError(String),
    BadRequest(String),
    NotFound(String),

    MissingToken,
    InvalidToken,
    ExpiredToken,
}

impl From<String> for ServiceError {
    fn from(error: String) -> Self {
        ServiceError::BadRequest(error)
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
            ServiceError::MissingToken => write!(f, "Missing token header"),
            ServiceError::InvalidToken => write!(f, "Invalid token"),
            ServiceError::ExpiredToken => write!(f, "Expired token, please refresh it"),
        }
    }
}

impl Debug for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
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
            ServiceError::MissingToken => {
                Response::<()>::new(StatusCode::UNAUTHORIZED, "Missing token header").into()
            }
            ServiceError::InvalidToken => {
                Response::<()>::new(StatusCode::UNAUTHORIZED, "Invalid token").into()
            }
            ServiceError::ExpiredToken => {
                Response::<()>::new(StatusCode::UNAUTHORIZED, "Expired token, please refresh it")
                    .into()
            }
        }
    }
}
