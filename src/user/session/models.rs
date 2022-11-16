use crate::constants::{EXPIRATION_TIME, ISSUER, REFRESH_EXPIRATION_TIME};
use crate::redis::client::{RedisClient, RedisKey};
use crate::utils::traits::RedisStorable;
use crate::utils::types::ServiceResult;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionClaims {
    pub username: String,
    pub device_id: String,

    pub exp: u32,
    pub iat: u32,
    pub iss: String,
    pub jti: String,
}

impl SessionClaims {
    pub fn new(username: String, device_id: String) -> Self {
        let now = chrono::Utc::now().timestamp() as u32;

        Self {
            username,
            device_id,
            exp: EXPIRATION_TIME + now,
            iat: now,
            iss: ISSUER.to_string(),
            jti: Uuid::new_v4().to_string(),
        }
    }
}

#[async_trait]
impl RedisStorable<SessionRefreshClaims> for SessionClaims {
    fn key(key: String) -> RedisKey {
        RedisKey::Session(key)
    }

    fn self_key(&self) -> RedisKey {
        RedisKey::Session(self.jti.clone())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionRefreshClaims {
    pub access_token_jti: String,

    pub exp: u32,
    pub iat: u32,
    pub iss: String,
    pub jti: String,
}

impl SessionRefreshClaims {
    pub fn new(access_token_jti: String) -> Self {
        let now = chrono::Utc::now().timestamp() as u32;

        Self {
            access_token_jti,
            exp: REFRESH_EXPIRATION_TIME + now,
            iat: now,
            iss: ISSUER.to_string(),
            jti: Uuid::new_v4().to_string(),
        }
    }
}

#[async_trait]
impl RedisStorable<SessionClaims> for SessionRefreshClaims {
    fn key(key: String) -> RedisKey {
        RedisKey::SessionRefresh(key)
    }

    fn self_key(&self) -> RedisKey {
        RedisKey::SessionRefresh(self.jti.clone())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthenticationTokens {
    pub access_token: String,
    pub refresh_token: String,

    pub session: SessionClaims,
    pub refresh_session: SessionRefreshClaims,
}

impl AuthenticationTokens {
    pub fn new(
        access_token: String,
        refresh_token: String,
        session: SessionClaims,
        refresh_session: SessionRefreshClaims,
    ) -> Self {
        Self {
            access_token,
            refresh_token,
            session,
            refresh_session,
        }
    }

    pub fn save(&self, redis: Arc<RedisClient>) -> ServiceResult<()> {
        self.session
            .save_other(&self.refresh_session, redis.clone())?;
        self.refresh_session
            .save_other(&self.session, redis.clone())?;

        Ok(())
    }

    pub async fn save_async(&self, redis: Arc<RedisClient>) -> ServiceResult<()> {
        self.session
            .save_other_async(&self.refresh_session, redis.clone())
            .await?;
        self.refresh_session
            .save_other_async(&self.session, redis.clone())
            .await?;

        Ok(())
    }
}
