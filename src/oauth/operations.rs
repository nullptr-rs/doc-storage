use oxide_auth::{
    endpoint::{
        AccessTokenFlow, AuthorizationFlow, ClientCredentialsFlow, Endpoint, RefreshFlow,
        ResourceFlow,
    },
    primitives::grant::Grant,
};

use crate::{
    api::{errors::ServiceError, responses::Response},
    oauth::{request::OAuthRequest},
};

pub trait OAuthOperation: Sized {
    type Item;

    fn run<E: Endpoint<OAuthRequest>>(self, endpoint: E) -> Result<Self::Item, ServiceError>;
}

#[derive(Debug, Clone)]
pub struct AccessTokenOperation {
    pub request: OAuthRequest,
}

impl OAuthOperation for AccessTokenOperation {
    type Item = Response<String>;

    fn run<E: Endpoint<OAuthRequest>>(self, endpoint: E) -> Result<Self::Item, ServiceError> {
        let mut prepared_flow =
            AccessTokenFlow::prepare(endpoint).map_err(|_| "Invalid access token operation")?;

        prepared_flow
            .execute(self.request)
            .map_err(|_| "Invalid access token operation".into())
    }
}

#[derive(Debug, Clone)]
pub struct AuthorizationOperation {
    pub request: OAuthRequest,
}

impl OAuthOperation for AuthorizationOperation {
    type Item = Response<String>;

    fn run<E: Endpoint<OAuthRequest>>(self, endpoint: E) -> Result<Self::Item, ServiceError> {
        let mut prepared_flow =
            AuthorizationFlow::prepare(endpoint).map_err(|_| "Invalid authorization operation")?;

        prepared_flow
            .execute(self.request)
            .map_err(|_| "Invalid authorization operation".into())
    }
}

#[derive(Debug, Clone)]
pub struct ClientCredentialsOperation {
    pub request: OAuthRequest,
}

impl OAuthOperation for ClientCredentialsOperation {
    type Item = Response<String>;

    fn run<E: Endpoint<OAuthRequest>>(self, endpoint: E) -> Result<Self::Item, ServiceError> {
        let mut prepared_flow = ClientCredentialsFlow::prepare(endpoint)
            .map_err(|_| "Invalid client credentials operation")?;

        prepared_flow
            .execute(self.request)
            .map_err(|_| "Invalid client credentials operation".into())
    }
}

#[derive(Debug, Clone)]
pub struct RefreshOperation {
    pub request: OAuthRequest,
}

impl OAuthOperation for RefreshOperation {
    type Item = Response<String>;

    fn run<E: Endpoint<OAuthRequest>>(self, endpoint: E) -> Result<Self::Item, ServiceError> {
        let mut prepared_flow =
            RefreshFlow::prepare(endpoint).map_err(|_| "Invalid refresh operation")?;

        prepared_flow
            .execute(self.request)
            .map_err(|_| "Invalid refresh operation".into())
    }
}

#[derive(Debug, Clone)]
pub struct ResourceOperation {
    pub request: OAuthRequest,
}

impl OAuthOperation for ResourceOperation {
    type Item = Grant;

    fn run<E: Endpoint<OAuthRequest>>(self, endpoint: E) -> Result<Self::Item, ServiceError> {
        let mut prepared_flow =
            ResourceFlow::prepare(endpoint).map_err(|_| "Invalid resource operation")?;

        prepared_flow
            .execute(self.request)
            .map_err(|_| "Invalid resource operation".into())
    }
}
