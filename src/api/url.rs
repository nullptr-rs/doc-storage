use actix_web::{Scope, web};
use crate::api::uploader;

pub fn register_url() -> Scope {
    Scope::new("/api")
        .service(
            web::resource("/upload")
                .route(web::post().to(uploader::upload))
        )
}