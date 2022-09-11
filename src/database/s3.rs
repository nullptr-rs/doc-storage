use rusoto_core::credential::StaticProvider;
use rusoto_core::{request, Region, RusotoError};
use rusoto_s3::{CreateBucketError, CreateBucketRequest, S3Client, S3};
use std::env;
use std::error::Error;

pub struct S3Database {
    pub client: S3Client,
    pub bucket: String,
}

pub async fn connection_builder() -> Result<S3Database, Box<dyn Error>> {
    let bucket_name = env::var("S3_BUCKET_NAME").expect("S3 bucket name not set");
    let storage_region = env::var("S3_REGION").expect("S3 region not set");
    let storage_url = env::var("S3_URL").expect("S3 url not set");
    let storage_access_key = env::var("S3_ACCESS_KEY").expect("S3 access key not set");
    let storage_secret_key = env::var("S3_SECRET_KEY").expect("S3 secret key not set");

    let client = S3Client::new_with(
        request::HttpClient::new()?,
        StaticProvider::new_minimal(storage_access_key, storage_secret_key),
        Region::Custom {
            name: storage_region,
            endpoint: storage_url,
        },
    );

    let create_bucket_request = CreateBucketRequest {
        bucket: bucket_name.clone(),
        ..Default::default()
    };
    match client.create_bucket(create_bucket_request).await {
        Ok(_) => println!("Bucket created"),
        Err(RusotoError::Service(CreateBucketError::BucketAlreadyOwnedByYou(_))) => {
            println!("Bucket already exists")
        }
        Err(e) => return Err(Box::new(e)),
    }

    Ok(S3Database {
        client,
        bucket: bucket_name,
    })
}
