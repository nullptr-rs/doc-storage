use crate::api::handler::{login, upload};
use actix_web::Scope;

pub fn register_endpoints() -> Scope {
    Scope::new("/api")
        .service(Scope::new("/v1"))
        .service(login::register_endpoints())
        .service(upload::register_endpoints())
}
