use crate::database::s3::S3Database;
use actix_web::HttpResponse;
use serde::Serialize;
use std::error::Error;

pub type AsyncHttpResponse = Result<HttpResponse, Box<dyn Error>>;

#[derive(Serialize)]
pub struct Response<T> {
    pub response: String,
    pub data: T,
}

#[derive(Serialize)]
pub struct ServerInfo {
    pub host: String,
    pub port: String,
    pub worker_threads: usize,
}

#[derive(Serialize)]
pub struct File {
    pub name: String,
    pub size: usize,
    #[serde(skip_serializing)]
    pub data: Vec<u8>,
}

impl From<Response<T>> for AsyncHttpResponse {
    fn from(response: Response<T>) -> Self {
        Ok(HttpResponse::Ok().json(response))
    }
}