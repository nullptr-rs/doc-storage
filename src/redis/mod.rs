use crate::api::utils::errors::ServiceError;
use redis::FromRedisValue;
use std::fmt::Display;
use serde::{Deserialize, Serialize};

pub struct RedisClient {
    pub client: redis::Client,
}

pub enum RedisKey {
    Base,
    Account(String),
    Session(String),
    Other(String),
}

impl RedisClient {
    pub fn new(connection_string: &str) -> Result<RedisClient, anyhow::Error> {
        let client = redis::Client::open(connection_string)?;

        let mut connection = client.get_connection()?;
        redis::cmd("PING")
            .query::<String>(&mut connection)
            .map_err(|error| anyhow::anyhow!(error))?;

        Ok(RedisClient { client })
    }

    pub async fn execute_raw<T: FromRedisValue>(
        &self,
        cmd: &mut redis::Cmd,
    ) -> Result<T, anyhow::Error> {
        let mut connection = self
            .client
            .get_async_connection()
            .await
            .map_err(|error| anyhow::anyhow!(error))?;

        cmd.query_async(&mut connection)
            .await
            .map_err(|error| anyhow::anyhow!(error))
    }

    pub async fn execute<T: FromRedisValue>(
        &self,
        cmd: &mut redis::Cmd,
    ) -> Result<T, ServiceError> {
        self.execute_raw(cmd).await.map_err(|error| {
            ServiceError::InternalServerError(
                "Failed to interact with the database".to_string(),
                Some(error.into()),
            )
        })
    }

    pub async fn async_get(&self, key: RedisKey) -> Result<String, ServiceError> {
        self.execute(redis::cmd("GET").arg(key.to_string())).await
    }

    pub async fn d_async_get<T: for<'a> Deserialize<'a>>(&self, key: RedisKey) -> Result<T, ServiceError> {
        let result = self.async_get(key).await?;

        let result = serde_json::from_str(&result).map_err(|error| {
            ServiceError::InternalServerError(
                "Failed to deserialize the data".to_string(),
                Some(error.into()),
            )
        })?;

        Ok(result)
    }

    pub async fn async_set(&self, key: RedisKey, value: &str) -> Result<String, ServiceError> {
        self.execute(
            redis::cmd("SET")
                .arg(key.to_string())
                .arg(value.to_string()),
        )
        .await
    }

    pub async fn s_async_set<T: Serialize>(&self, key: RedisKey, value: &T) -> Result<String, ServiceError> {
        let value = serde_json::to_string(value).map_err(|error| {
            ServiceError::InternalServerError(
                "Failed to serialize the data".to_string(),
                Some(error.into()),
            )
        })?;

        self.async_set(key, &value).await
    }

    pub async fn async_exists(&self, key: RedisKey) -> Result<bool, ServiceError> {
        self.execute(redis::cmd("EXISTS").arg(key.to_string()))
            .await
    }

    pub async fn async_del(&self, key: RedisKey) -> Result<String, ServiceError> {
        self.execute(redis::cmd("DEL").arg(key.to_string())).await
    }
}

impl Display for RedisKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RedisKey::Base => write!(f, "doc_storage"),
            RedisKey::Account(username) => write!(f, "{}:account:{}", RedisKey::Base, username),
            RedisKey::Session(session_id) => write!(f, "{}:session:{}", RedisKey::Base, session_id),
            RedisKey::Other(key) => write!(f, "{}:{}", RedisKey::Base, key),
        }
    }
}
