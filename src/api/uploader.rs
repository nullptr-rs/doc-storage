use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{HttpResponse, Scope, web};
use crate::api::types::{AsyncHttpResponse, Databases};
use crate::api::uploader;

//Upload a file and upload it in s3
pub async fn upload(databases: web::Data<Databases>, mut payload: Multipart) -> AsyncHttpResponse {
    let (mut file_name, mut file_data) = get_file(&mut payload, "multipart/form-data").await?;
    let &mut s3_client = databases.s3.client;

    let result = s3_client.put_object().bucket("bucket").key("key").body(file).send().await;
}

pub async fn get_file(payload: &mut Multipart, content_type: &str) -> Result<(String, Vec<u8>), MultipartError> {
    let mut data = Vec::new();

    let mut file = payload.try_next().await?.ok_or(MultipartError::Incomplete)?;
    let mut file_name = file.name().to_string();

    let content_type = file.content_type().ok_or(MultipartError::Incomplete)?;
    if content_type != content_type {
        return Err(MultipartError::Incomplete);
    }

    while let Some(chunk) = file.try_next().await? {
        data.extend_from_slice(&chunk);
    }

    Ok((file_name, data))
}