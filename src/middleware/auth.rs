use crate::conditional;
use crate::constants::{BASE_ROUTE, IGNORED_AUTH_ROUTES};
use crate::redis::client::RedisClient;
use crate::user::session::models::SessionClaims;
use crate::user::session::token;
use crate::utils::api::errors::ServiceError;
use crate::utils::traits::RedisStorable;
use actix_web::body::MessageBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::web::Data;
use actix_web::HttpMessage;
use futures::future::{ready, Future, Ready};
use std::pin::Pin;
use std::sync::Arc;
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
        if self.check_required_auth(&req) {
            let auth_header = req.headers().get("Authorization");
            conditional!(
                auth_header.is_none(),
                return self.failure(ServiceError::MissingToken)
            );

            let auth_header = auth_header.unwrap().to_str().unwrap();
            conditional!(
                auth_header.is_empty() || !auth_header.starts_with("Bearer"),
                return self.failure(ServiceError::MissingToken)
            );

            let token = auth_header.replace("Bearer ", "");
            conditional!(
                token.is_empty(),
                return self.failure(ServiceError::MissingToken)
            );

            let validation = token::decode_access_token(token.clone());
            conditional!(
                validation.is_err(),
                return self.failure(validation.err().unwrap())
            );

            let validation = validation.unwrap();

            let redis = req.app_data::<Data<Arc<RedisClient>>>().unwrap();
            let exists = SessionClaims::exists(validation.jti.clone(), redis.get_ref().clone());
            conditional!(exists.is_err(), return self.failure(exists.err().unwrap()));

            let exists = exists.unwrap();
            conditional!(!exists, return self.failure(ServiceError::InvalidToken));

            req.extensions_mut().insert(token.clone());
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

    pub fn check_required_auth(&self, req: &ServiceRequest) -> bool {
        let bypass_auth = IGNORED_AUTH_ROUTES.iter().any(|route| {
            let path = format!("{}/{}", BASE_ROUTE, route);

            req.path().starts_with(path.as_str())
        });

        !bypass_auth
    }

    pub fn failure(&self, error: ServiceError) -> ServiceFuture<B> {
        Box::pin(async move { Err(error.into()) })
    }
}
