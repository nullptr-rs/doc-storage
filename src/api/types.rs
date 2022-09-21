use std::error::Error;
use actix_web::HttpResponse;
use crate::database::s3::S3Database;

pub type AsyncHttpResponse = Result<HttpResponse, Box<dyn Error>>;

pub struct Databases {
    pub s3: S3Database
}