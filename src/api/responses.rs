use actix_web::{
    http::{header::HeaderMap, StatusCode},
    HttpResponse,
};
use oxide_auth::{endpoint::WebResponse, frontends::dev::Url};
use serde::Serialize;

use crate::api::errors::ServiceError;

#[derive(Clone, Debug, Serialize)]
pub struct Response<T> {
    pub status: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing)]
    pub headers: Option<HeaderMap>,
}

impl<T> Response<T> {
    pub fn new(status: StatusCode, message: &str) -> Self {
        Self {
            status: status.as_u16(),
            message: message.to_string(),
            data: None,
            error: None,
            headers: None,
        }
    }

    pub fn status(&mut self, status: StatusCode) {
        self.status = status.as_u16();
    }

    pub fn data(&mut self, data: T) {
        self.data = Some(data);
    }

    pub fn error(&mut self, error: anyhow::Error) {
        self.error = Some(error.to_string());
    }

    pub fn error_message(&mut self, error: String) {
        self.error = Some(error);
    }

    pub fn header(&mut self, key: &str, value: &str) {
        let headers = self.headers.get_or_insert(HeaderMap::new());
        headers.insert(key.parse().unwrap(), value.parse().unwrap());

        self.headers = Some(headers.clone());
    }

    pub fn headers(&mut self, headers: HeaderMap) {
        self.headers = Some(headers);
    }
}

impl WebResponse for Response<String> {
    type Error = ServiceError;

    fn ok(&mut self) -> Result<(), Self::Error> {
        self.status(StatusCode::OK);

        Ok(())
    }

    fn redirect(&mut self, url: Url) -> Result<(), Self::Error> {
        self.status(StatusCode::FOUND);
        self.header("Location", &url.to_string());

        Ok(())
    }

    fn client_error(&mut self) -> Result<(), Self::Error> {
        self.status(StatusCode::BAD_REQUEST);

        Ok(())
    }

    fn unauthorized(&mut self, header_value: &str) -> Result<(), Self::Error> {
        self.status(StatusCode::UNAUTHORIZED);
        self.header("WWW-Authenticate", header_value);

        Ok(())
    }

    fn body_text(&mut self, text: &str) -> Result<(), Self::Error> {
        self.status(StatusCode::OK);

        self.header("Content-Type", "text/plain");
        self.data(text.to_string());

        Ok(())
    }

    fn body_json(&mut self, data: &str) -> Result<(), Self::Error> {
        self.status(StatusCode::OK);

        self.header("Content-Type", "application/json");
        self.data(data.to_string());

        Ok(())
    }
}

impl<T: Serialize> From<Response<T>> for HttpResponse {
    fn from(response: Response<T>) -> Self {
        let code = StatusCode::from_u16(response.status)
            .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR);

        let mut http_response = HttpResponse::build(code);

        if let Some(headers) = &response.headers {
            for (key, value) in headers.iter() {
                http_response.append_header((key, value));
            }
        }

        http_response.json(response)
    }
}
