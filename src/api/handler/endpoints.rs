use crate::api::handler::{authentication, upload};
use actix_web::Scope;

pub fn register_endpoints() -> Scope {
    Scope::new("/api").service(
        Scope::new("/v1")
            .service(authentication::register_endpoints())
            .service(upload::register_endpoints()),
    )
}
