use actix_web::{Scope, web};

pub fn register_url() -> Scope {
    Scope::new("/api")
        .service(
            web::resource("/upload")
                .route(web::post().to(upload::upload))
                .route(web::get().to(upload::get_upload))
        )
}