use actix_web::rt::System;
use actix_web::{App, HttpServer};
use doc_storage::database::s3;
use doc_storage::storage::chunk;
use rusoto_s3::S3;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::time::SystemTime;
use tokio::fs;
use tokio::runtime::Builder;

fn main() -> Result<(), Box<dyn Error>> {
    let worker_threads = env::var("WORKER_THREADS")
        .unwrap_or_else(|_| "8".to_string())
        .parse::<usize>()
        .unwrap();

    System::with_tokio_rt(|| {
        Builder::new_multi_thread()
            .thread_name("doc-storage-worker")
            .worker_threads(worker_threads)
            .build()
            .unwrap()
    })
    .block_on(async_bootstrap())
}

async fn async_bootstrap() -> Result<(), Box<dyn Error>> {
    print!("Hello, world!");

    HttpServer::new(move || App::new())
        .bind("127.0.0.1:1109")?
        .run()
        .await?;

    Ok(())
    //https://lib.rs/crates/bita
    //https://discord.com/channels/648981252988338213/935847071540469820/1016443689679200286
    //TODO Cache dependencies in Github CI and DockerFile, Add SHA256 checksum to chunks
}
