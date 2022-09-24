use rusoto_core::credential::StaticProvider;
use rusoto_core::{HttpClient, Region, RusotoError};
use rusoto_s3::{CreateBucketError, CreateBucketRequest, S3Client, S3};
use std::env;

pub struct S3Database {
    pub client: S3Client,
    pub bucket: String,
}

impl S3Database {
    pub fn new() -> Self {
        let bucket = env::var("S3_BUCKET").expect("S3 bucket must be set");

        let region = env::var("S3_REGION").expect("S3 region must be set");
        let url = env::var("S3_URL").expect("S3 url must be set");
        let region = Region::Custom {
            name: region,
            endpoint: url,
        };

        let access_key = env::var("S3_ACCESS_KEY").expect("S3 access key must be set");
        let secret_key = env::var("S3_SECRET_KEY").expect("S3 secret key must be set");
        let credentials = StaticProvider::new_minimal(access_key, secret_key);

        let http_client = HttpClient::new().expect("Failed to create S3 request dispatcher");
        let client = S3Client::new_with(http_client, credentials, region);

        Self { client, bucket }
    }

    pub async fn create_bucket(&self) -> Result<(), RusotoError<CreateBucketError>> {
        let request = CreateBucketRequest {
            bucket: self.bucket.clone(),
            ..Default::default()
        };

        match self.client.create_bucket(request).await {
            Err(RusotoError::Service(CreateBucketError::BucketAlreadyOwnedByYou(_))) => {
                println!("Bucket already exists, skipping creation...");
                Ok(())
            }
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
