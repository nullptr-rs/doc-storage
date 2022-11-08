use crate::api::utils::errors::ServiceError;
use crate::conditional_return;
use crate::constants::{BASE_ROUTE, IGNORED_AUTH_ROUTES};
use crate::jwt::token;
use actix_web::body::MessageBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::HttpMessage;
use futures::future::{ready, Future, Ready};
use jsonwebtoken::errors::ErrorKind;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct AuthenticationMiddleware;
pub struct AuthenticationMiddlewareService<S> {
    service: S,
}

pub type ServiceFuture<B> =
    Pin<Box<dyn Future<Output = Result<ServiceResponse<B>, actix_web::Error>>>>;

impl<S, B> Transform<S, ServiceRequest> for AuthenticationMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = AuthenticationMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddlewareService::new(service)))
    }
}

impl AuthenticationMiddleware {
    pub fn new() -> Self {
        Self {}
    }
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = ServiceFuture<B>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let bypass_auth = IGNORED_AUTH_ROUTES.iter().any(|route| {
            req.path()
                .starts_with(format!("{}/{}", BASE_ROUTE, route).as_str())
        });

        if !bypass_auth {
            let auth_header = req.headers().get("Authorization");
            conditional_return!(
                auth_header.is_none(),
                self.failure(ServiceError::MissingToken)
            );

            let auth_header = auth_header.unwrap().to_str().unwrap();
            conditional_return!(
                auth_header.is_empty() || !auth_header.starts_with("Bearer"),
                self.failure(ServiceError::MissingToken)
            );

            let token = auth_header.replace("Bearer ", "");
            conditional_return!(token.is_empty(), self.failure(ServiceError::MissingToken));

            let validation = token::decode_token(&token).map_err(|error| {
                let error = error.into_kind();

                match error {
                    ErrorKind::ExpiredSignature => ServiceError::ExpiredToken,
                    _ => ServiceError::InvalidToken,
                }
            });
            conditional_return!(validation.is_err(), self.failure(validation.err().unwrap()));

            let validation = validation.unwrap();
            req.extensions_mut().insert(validation);

            log::debug!("AuthenticationMiddleware: {:?}", token);
        }

        let future = self.service.call(req);
        Box::pin(async move {
            let res = future.await?;
            Ok(res)
        })
    }
}

impl<S, B> AuthenticationMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    pub fn new(service: S) -> Self {
        AuthenticationMiddlewareService { service }
    }

    pub fn failure(&self, error: ServiceError) -> ServiceFuture<B> {
        Box::pin(async move { Err(error.into()) })
    }
}
