use actix_web::middleware::{Compress, Logger};
use actix_web::rt::System;
use actix_web::web::Data;
use actix_web::{App, HttpServer, Scope};
use doc_storage::middleware::auth::AuthenticationMiddleware;
use doc_storage::redis::client::RedisClient;
use doc_storage::user;
use std::env;
use std::sync::Arc;
use tokio::runtime::Builder;

fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let worker_threads = env::var("WORKER_THREADS")
        .unwrap_or_else(|_| "8".to_string())
        .parse::<usize>()
        .unwrap();

    log::info!("Starting server with {} worker threads...", worker_threads);

    System::with_tokio_rt(|| {
        Builder::new_multi_thread()
            .thread_name("doc-storage-worker")
            .worker_threads(worker_threads)
            .enable_all()
            .build()
            .expect("Failed to create Tokio runtime")
    })
    .block_on(async_bootstrap(worker_threads))
}

async fn async_bootstrap(worker_threads: usize) -> std::io::Result<()> {
    let host = env::var("DOC_STORAGE_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("DOC_STORAGE_PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("{}:{}", host, port);

    let redis_host = env::var("REDIS_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let redis_port = env::var("REDIS_PORT").unwrap_or_else(|_| "6379".to_string());
    let redis_password = env::var("REDIS_PASSWORD").unwrap_or_else(|_| "nullptr-rs".to_string());
    let redis_address = format!("redis://:{}@{}:{}", redis_password, redis_host, redis_port);

    let redis = Arc::new(
        RedisClient::new(redis_address).expect("Failed to connect to Redis. Is it running?"),
    );

    log::info!("Starting server on {}...", &address);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Compress::default())
            .wrap(AuthenticationMiddleware::new())
            .app_data(Data::new(redis.clone()))
            .service(
                Scope::new("/api")
                    .service(Scope::new("/v1").service(user::api::handlers::register_endpoints())),
            )
    })
    .workers(worker_threads)
    .bind(address)?
    .run()
    .await
}

/*
pub async fn extract_files(payload: &mut Multipart) -> Result<Vec<File>, MultipartError> {
    let mut files = Vec::new();

    log::info!("Iterating files...");
    while let Some(mut field) = payload.try_next().await? {
        let mut data = Vec::new();

        log::info!("Getting file...");
        let file_name = field.name().to_string();
        log::info!("File name: {}", file_name);

        log::info!("Reading file...");
        while let Some(chunk) = field.try_next().await? {
            log::info!("Getting chunk: {}", chunk.len());
            data.extend_from_slice(&chunk);
        }
        log::info!("File read: {}", data.len());

        files.push(File {
            name: file_name,
            size: data.len(),
            data,
        });
    }

    log::info!("Files: {}", files.len());

    Ok(files)
}
 */
