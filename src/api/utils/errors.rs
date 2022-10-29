use crate::api::utils::types::Response;
use crate::conditional;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use std::fmt::{Debug, Display, Formatter};

pub enum ServiceError {
    InternalServerError(String, Option<anyhow::Error>),
    BadRequest(String),
    NotFound(String),
    Unauthorized,
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceError::InternalServerError(message, _) => {
                write!(f, "Internal server error: {}", message)
            }
            ServiceError::BadRequest(message) => write!(f, "Bad request: {}", message),
            ServiceError::NotFound(message) => write!(f, "Resource not found: {}", message),
            ServiceError::Unauthorized => write!(f, "Unauthorized"),
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
            ServiceError::InternalServerError(_, _) => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ServiceError::NotFound(_) => StatusCode::NOT_FOUND,
            ServiceError::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let response: Response<()> = self.into();
        response.into()
    }
}

impl From<ServiceError> for Response<()> {
    fn from(error: ServiceError) -> Self {
        match error {
            ServiceError::InternalServerError(message, error) => {
                let mut response = Response::new(StatusCode::INTERNAL_SERVER_ERROR, &message);

                conditional!(error.is_some(), {
                    response = response.error(error.unwrap());
                });

                response
            }
            ServiceError::BadRequest(message) => Response::new(StatusCode::BAD_REQUEST, &message),
            ServiceError::NotFound(message) => Response::new(StatusCode::NOT_FOUND, &message),
            ServiceError::Unauthorized => Response::new(StatusCode::UNAUTHORIZED, "Unauthorized"),
        }
    }
}

impl From<&ServiceError> for Response<()> {
    fn from(error: &ServiceError) -> Self {
        match error {
            ServiceError::InternalServerError(message, error) => {
                let mut response = Response::new(StatusCode::INTERNAL_SERVER_ERROR, message);

                conditional!(error.is_some(), {
                    let error = error.as_ref().unwrap();
                    response = response.error_message(error.to_string());
                });

                response
            }
            ServiceError::BadRequest(message) => Response::new(StatusCode::BAD_REQUEST, message),
            ServiceError::NotFound(message) => Response::new(StatusCode::NOT_FOUND, message),
            ServiceError::Unauthorized => Response::new(StatusCode::UNAUTHORIZED, "Unauthorized"),
        }
    }
}

impl From<ServiceError> for HttpResponse {
    fn from(error: ServiceError) -> Self {
        let response: Response<()> = error.into();

        response.into()
    }
}

impl From<&ServiceError> for HttpResponse {
    fn from(error: &ServiceError) -> Self {
        let response: Response<()> = error.into();

        response.into()
    }
}
