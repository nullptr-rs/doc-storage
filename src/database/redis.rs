use redis::{Client, RedisResult};
use std::env;

pub fn connection_builder() -> RedisResult<Client> {
    let redis_user = env::var("REDIS_USER").unwrap_or_else(|_| "".to_string());
    let redis_password = env::var("REDIS_PASSWORD").unwrap_or_else(|_| "".to_string());
    let redis_port = env::var("REDIS_PORT").unwrap_or_else(|_| "6379".to_string());
    let redis_db = env::var("REDIS_DB").unwrap_or_else(|_| "0".to_string());
    let redis_host = env::var("REDIS_HOST").expect("Redis host not set");

    Client::open(format!(
        "redis://{}:{}@{}:{}/{}",
        redis_user, redis_password, redis_host, redis_port, redis_db
    ))
}
