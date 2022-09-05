use std::env;

pub fn connection_builder() -> Result<redis::Client, redis::RedisError> {
    let redis_user = env::var("REDIS_USER").unwrap_or_else("".to_string());
    let redis_password = env::var("REDIS_PASSWORD").unwrap_or_else("".to_string());
    let redis_port = env::var("REDIS_PORT").unwrap_or_else("6379".to_string());
    let redis_db = env::var("REDIS_DB").unwrap_or_else("0".to_string());
    let redis_host = env::var("REDIS_HOST").expect("Redis host not set");

    redis::Client::open(format!(
        "redis://{}:{}@{}:{}/{}",
        redis_user, redis_password, redis_host, redis_port, redis_db
    ))
}
