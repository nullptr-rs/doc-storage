use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{
    database::redis::{RedisClient, RedisKey},
    utils::types::ServiceResult,
};

#[async_trait]
pub trait RedisStorable<T: Sized + Sync + Send + Serialize + for<'a> Deserialize<'a>> {
    fn key(key: String) -> RedisKey;
    fn self_key(&self) -> RedisKey;

    fn fetch(key: String, redis: Arc<RedisClient>) -> ServiceResult<T> {
        let redis_key = Self::key(key);
        let value = redis.d_get(redis_key)?;

        Ok(value)
    }

    async fn fetch_async(key: String, redis: Arc<RedisClient>) -> ServiceResult<T> {
        let redis_key = Self::key(key);
        let value = redis.d_async_get(redis_key).await?;

        Ok(value)
    }

    fn save(&self, redis: Arc<RedisClient>) -> ServiceResult<()>
    where
        Self: Sized + Serialize,
    {
        let redis_key = self.self_key();
        redis.s_set(redis_key, self)?;

        Ok(())
    }

    async fn save_async(&self, redis: Arc<RedisClient>) -> ServiceResult<()>
    where
        Self: Sized + Serialize,
    {
        let redis_key = self.self_key();
        redis.s_async_set(redis_key, self).await?;

        Ok(())
    }

    fn exists(key: String, redis: Arc<RedisClient>) -> ServiceResult<bool> {
        let redis_key = Self::key(key);
        let exists = redis.exists(redis_key)?;

        Ok(exists)
    }

    async fn exists_async(key: String, redis: Arc<RedisClient>) -> ServiceResult<bool> {
        let redis_key = Self::key(key);
        let exists = redis.async_exists(redis_key).await?;

        Ok(exists)
    }

    fn delete(&self, redis: Arc<RedisClient>) -> ServiceResult<()> {
        let redis_key = self.self_key();
        redis.delete(redis_key)?;

        Ok(())
    }

    async fn delete_async(&self, redis: Arc<RedisClient>) -> ServiceResult<()> {
        let redis_key = self.self_key();
        redis.async_delete(redis_key).await?;

        Ok(())
    }

    fn expire(&self, redis: Arc<RedisClient>, seconds: u32) -> ServiceResult<()> {
        let redis_key = self.self_key();
        redis.expire(redis_key, seconds)?;

        Ok(())
    }

    async fn expire_async(&self, redis: Arc<RedisClient>, seconds: u32) -> ServiceResult<()> {
        let redis_key = self.self_key();
        redis.async_expire(redis_key, seconds).await?;

        Ok(())
    }
}
