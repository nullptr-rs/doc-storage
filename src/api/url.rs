use crate::api::uploader;
use actix_web::{web, Scope};

pub fn register_url() -> Scope {
    Scope::new("/api").service(web::resource("/upload").route(web::post().to(uploader::upload)))
}
