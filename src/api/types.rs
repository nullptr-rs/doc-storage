use crate::database::s3::S3Database;
use actix_web::HttpResponse;
use serde::Serialize;
use std::error::Error;

pub type AsyncHttpResponse = Result<HttpResponse, Box<dyn Error>>;

pub struct Databases {
    pub s3: S3Database,
}

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

impl Databases {
    pub fn new(s3: S3Database) -> Self {
        Databases { s3 }
    }
}

impl ServerInfo {
    pub fn new(host: String, port: String, worker_threads: usize) -> Self {
        ServerInfo {
            host,
            port,
            worker_threads,
        }
    }
}

impl File {
    pub fn new(name: String, size: usize, data: Vec<u8>) -> Self {
        File { name, size, data }
    }
}
