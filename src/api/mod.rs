use crate::api::handler::{login, upload};
use actix_web::{web, Scope};
use crate::utils::constants;

pub mod handler;
pub mod utils;

pub fn register_endpoints() -> Scope {
    Scope::new(constants::BASE_ROUTE)
        .service(
            Scope::new("/auth")
                .service(
                    web::resource("/register").route(web::post().to(login::handle_registration)),
                )
                .service(web::resource("/login").route(web::post().to(login::handle_login)))
                .service(web::resource("/logout").route(web::post().to(login::handle_logout))),
        )
        .service(web::resource("/upload").route(web::post().to(upload::handle_file_upload)))
}
