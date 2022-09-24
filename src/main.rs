use actix_web::rt::System;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use doc_storage::api::types::{Databases, ServerInfo};
use doc_storage::api::url;
use doc_storage::database::s3::S3Database;
use std::env;
use tokio::runtime::Builder;

fn main() -> std::io::Result<()> {
    let worker_threads = env::var("WORKER_THREADS")
        .unwrap_or_else(|_| "8".to_string())
        .parse::<usize>()
        .unwrap();

    print!(
        "Starting server with {} worker threads...\n",
        worker_threads
    );

    env::set_var("S3_BUCKET", "doc-storage");
    env::set_var("S3_REGION", "eu-west-2");
    env::set_var("S3_URL", "http://127.0.0.1:9000");
    env::set_var("S3_ACCESS_KEY", "doc-storage-user");
    env::set_var("S3_SECRET_KEY", "doc-storage-password");

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
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("{}:{}", host.clone(), port.clone());

    let app_info = Data::new(ServerInfo::new(host, port, worker_threads));

    let s3 = S3Database::new();
    s3.create_bucket().await.expect("Failed to create bucket");

    let databases = Data::new(Databases::new(s3));

    print!("Starting server on {}...\n", &address);

    HttpServer::new(move || {
        App::new()
            .app_data(app_info.to_owned())
            .app_data(databases.to_owned())
            .service(url::register_url())
    })
    .workers(worker_threads)
    .bind(address)?
    .run()
    .await
    //https://lib.rs/crates/bita
    //https://discord.com/channels/648981252988338213/935847071540469820/1016443689679200286
    //TODO Add SHA256 checksum to chunks
}
