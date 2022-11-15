use crate::api::utils::errors::ServiceError;
use crate::api::utils::types::ServiceResult;
use redis::FromRedisValue;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub struct RedisClient {
    pub client: redis::Client,
}

impl RedisClient {
    pub fn new(connection_string: String) -> Result<RedisClient, anyhow::Error> {
        let client = redis::Client::open(connection_string)?;

        let mut connection = client.get_connection()?;
        redis::cmd("PING")
            .query::<String>(&mut connection)
            .map_err(|error| anyhow::anyhow!(error))?;

        Ok(RedisClient { client })
    }

    pub fn execute_raw<T: FromRedisValue>(&self, cmd: &mut redis::Cmd) -> Result<T, anyhow::Error> {
        let mut connection = self
            .client
            .get_connection()
            .map_err(|error| anyhow::anyhow!(error))?;

        cmd.query(&mut connection)
            .map_err(|error| anyhow::anyhow!(error))
    }

    pub async fn execute_async_raw<T: FromRedisValue>(
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

    pub fn execute<T: FromRedisValue>(&self, cmd: &mut redis::Cmd) -> ServiceResult<T> {
        self.execute_raw(cmd)
            .map_err(|_| ServiceError::redis_query())
    }

    pub async fn execute_async<T: FromRedisValue>(&self, cmd: &mut redis::Cmd) -> ServiceResult<T> {
        self.execute_async_raw(cmd)
            .await
            .map_err(|_| ServiceError::redis_query())
    }

    pub fn get(&self, key: RedisKey) -> ServiceResult<String> {
        self.execute(redis::cmd("GET").arg(key.to_string()))
    }

    pub async fn async_get(&self, key: RedisKey) -> ServiceResult<String> {
        self.execute_async(redis::cmd("GET").arg(key.to_string()))
            .await
    }

    pub fn d_get<T: for<'a> Deserialize<'a>>(&self, key: RedisKey) -> ServiceResult<T> {
        let value = self.get(key)?;

        serde_json::from_str(&value).map_err(|_| ServiceError::deserialization())
    }

    pub async fn d_async_get<T: for<'a> Deserialize<'a>>(&self, key: RedisKey) -> ServiceResult<T> {
        let value = self.async_get(key).await?;

        serde_json::from_str(&value).map_err(|_| ServiceError::deserialization())
    }

    pub fn set(&self, key: RedisKey, value: &str) -> ServiceResult<String> {
        self.execute(
            redis::cmd("SET")
                .arg(key.to_string())
                .arg(value.to_string()),
        )
    }

    pub async fn async_set(&self, key: RedisKey, value: &str) -> ServiceResult<String> {
        self.execute_async(
            redis::cmd("SET")
                .arg(key.to_string())
                .arg(value.to_string()),
        )
        .await
    }

    pub fn s_set<T: Serialize>(&self, key: RedisKey, value: &T) -> ServiceResult<String> {
        let value = serde_json::to_string(value).map_err(|_| ServiceError::serialization())?;

        self.set(key, &value)
    }

    pub async fn s_async_set<T: Serialize>(
        &self,
        key: RedisKey,
        value: &T,
    ) -> ServiceResult<String> {
        let value = serde_json::to_string(value).map_err(|_| ServiceError::serialization())?;

        self.async_set(key, &value).await
    }

    pub fn exists(&self, key: RedisKey) -> ServiceResult<bool> {
        self.execute(redis::cmd("EXISTS").arg(key.to_string()))
    }

    pub async fn async_exists(&self, key: RedisKey) -> ServiceResult<bool> {
        self.execute_async(redis::cmd("EXISTS").arg(key.to_string()))
            .await
    }

    pub fn delete(&self, key: RedisKey) -> ServiceResult<String> {
        self.execute(redis::cmd("DEL").arg(key.to_string()))
    }

    pub async fn async_delete(&self, key: RedisKey) -> ServiceResult<String> {
        self.execute_async(redis::cmd("DEL").arg(key.to_string()))
            .await
    }

    pub fn expire(&self, key: RedisKey, seconds: u32) -> ServiceResult<String> {
        self.execute(redis::cmd("EXPIRE").arg(key.to_string()).arg(seconds))
    }

    pub async fn async_expire(&self, key: RedisKey, seconds: u32) -> ServiceResult<String> {
        self.execute_async(redis::cmd("EXPIRE").arg(key.to_string()).arg(seconds))
            .await
    }
}

pub enum RedisKey {
    Base,
    Account(String),
    Session(String),
    Other(String),
}

impl Display for RedisKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RedisKey::Base => write!(f, "doc_storage"),
            RedisKey::Account(username) => write!(f, "{}:account:{}", RedisKey::Base, username),
            RedisKey::Session(username) => write!(f, "{}:session:{}", RedisKey::Base, username),
            RedisKey::Other(key) => write!(f, "{}:{}", RedisKey::Base, key),
        }
    }
}
