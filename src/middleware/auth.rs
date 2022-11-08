use std::task::{Context, Poll};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use futures::future::{LocalBoxFuture, Ready, ready};
use crate::utils::constants::{BASE_ROUTE, IGNORED_AUTH_ROUTES};

pub struct Authentication;
pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Transform<S, ServiceRequest> for Authentication
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware {
            service
        }))
    }
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let mut authentication_passed = false;

        for ignored_route in IGNORED_AUTH_ROUTES.iter() {
            if req.path().starts_with(format!("{}/{}", BASE_ROUTE, ignored_route).as_str()) {
                authentication_passed = true;
                break;
            }
        }

        todo!("Check if user is authenticated");
    }
}