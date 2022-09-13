use doc_storage::database::s3;
use doc_storage::storage::chunk;
use rusoto_s3::S3;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::time::SystemTime;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    /*println!("Connecting to S3...");

    env::set_var("S3_BUCKET_NAME", "doc-storage");
    env::set_var("S3_REGION", "eu-west-2");
    env::set_var("S3_URL", "http://127.0.0.1:9000");
    env::set_var("S3_ACCESS_KEY", "doc-storage-user");
    env::set_var("S3_SECRET_KEY", "doc-storage-password");
    let client = s3::connection_builder().await?;

    println!("Uploading Cargo.lock...");
    let put_object_request = rusoto_s3::PutObjectRequest {
        bucket: client.bucket.clone(),
        key: "Cargo.lock".to_string(),
        body: Some(fs::read("Cargo.lock").await.unwrap().into()),
        ..Default::default()
    };
    client.client.put_object(put_object_request).await?;
    println!("Uploaded Cargo.lock");*/

    print!("Hello, world!");
    Ok(())

    //https://lib.rs/crates/bita
    //https://discord.com/channels/648981252988338213/935847071540469820/1016443689679200286
    //TODO Cache dependencies in Github CI and DockerFile, Add SHA256 checksum to chunks
}
