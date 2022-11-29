use std::borrow::Cow;

use actix_web::{
    dev::Payload,
    web::{Form, Query},
    FromRequest, HttpRequest,
};
use oxide_auth::{
    endpoint::{NormalizedParameter, QueryParameter, WebRequest},
    frontends::dev::Url,
};

use crate::{
    api::{errors::ServiceError, responses::Response},
    utils::types::ServiceResult,
};

#[derive(Clone, Debug)]
pub struct OAuthRequest {
    request: HttpRequest,

    query: NormalizedParameter,
    body: NormalizedParameter,
    auth_header: String,
}

impl OAuthRequest {
    pub async fn new(request: HttpRequest, mut payload: Payload) -> ServiceResult<Self> {
        let query = Query::<NormalizedParameter>::from_request(&request, &mut payload)
            .await
            .map(|q| q.into_inner())
            .map_err(|_| "Invalid query")?;
        let body = Form::<NormalizedParameter>::from_request(&request, &mut payload)
            .await
            .map(|b| b.into_inner())
            .map_err(|_| "Invalid body")?;
        let auth_header = request
            .headers()
            .get("Authorization")
            .map(|h| h.to_str().unwrap().to_string())
            .unwrap_or_default();

        Ok(Self {
            request,
            query,
            body,
            auth_header,
        })
    }

    pub fn get_url(&self) -> ServiceResult<Url> {
        let url = self.request.uri().to_string();
        Url::parse(&url).map_err(|_| ServiceError::InternalServerError("Invalid URL".to_string()))
    }
}

impl WebRequest for OAuthRequest {
    type Error = ServiceError;
    type Response = Response<String>;

    fn query(&mut self) -> Result<Cow<dyn QueryParameter + 'static>, Self::Error> {
        Ok(Cow::Borrowed(&self.query))
    }

    fn urlbody(&mut self) -> Result<Cow<dyn QueryParameter + 'static>, Self::Error> {
        Ok(Cow::Borrowed(&self.body))
    }

    fn authheader(&mut self) -> Result<Option<Cow<str>>, Self::Error> {
        Ok(Some(Cow::Borrowed(&self.auth_header)))
    }
}
