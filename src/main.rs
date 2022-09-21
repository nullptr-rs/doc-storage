use actix_web::rt::System;
use actix_web::{web, App, HttpServer};
use std::env;
use actix_web::web::Data;
use tokio::runtime::Builder;

fn main() -> std::io::Result<()> {
    let worker_threads = env::var("WORKER_THREADS")
        .unwrap_or_else(|_| "8".to_string())
        .parse::<usize>()
        .unwrap();

    print!("Starting server with {} worker threads...\n", worker_threads);

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
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let address = format!("{}:{}", host, port);

    print!("Starting server on {}...\n", &address);

    HttpServer::new(|| {
        App::new()
            .app_data(Data::new("Hello world"))
                .route("/", web::get().to(|| {
                    async { "Hello world!" }
                }))
    })
        .workers(worker_threads)
        .bind(&address)?
        .run()
        .await
    //https://lib.rs/crates/bita
    //https://discord.com/channels/648981252988338213/935847071540469820/1016443689679200286
    //TODO Add SHA256 checksum to chunks
}
