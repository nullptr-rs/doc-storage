use crate::api::utils::errors::ServiceError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::Serialize;

pub type AccessToken = String;
pub type RefreshToken = String;
pub type ServiceResult<T> = Result<T, ServiceError>;

#[derive(Serialize)]
pub struct Response<T> {
    pub status: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T> Response<T> {
    pub fn new(status: StatusCode, message: &str) -> Self {
        Self {
            status: status.as_u16(),
            message: message.to_string(),
            data: None,
            error: None,
        }
    }

    pub fn data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    pub fn error(mut self, error: anyhow::Error) -> Self {
        self.error = Some(error.to_string());
        self
    }

    pub fn error_message(mut self, error: String) -> Self {
        self.error = Some(error);
        self
    }
}

impl<T: Serialize> From<Response<T>> for HttpResponse {
    fn from(response: Response<T>) -> Self {
        let code = StatusCode::from_u16(response.status)
            .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR);

        HttpResponse::build(code).json(response)
    }
}
