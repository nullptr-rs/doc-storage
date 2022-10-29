use crate::api::utils::types::Response;
use crate::utils::types::File;
use actix_multipart::{Multipart, MultipartError};
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use futures::stream::TryStreamExt;

pub async fn handle_file_upload(mut payload: Multipart) -> HttpResponse {
    let files = extract_files(&mut payload)
        .await
        .expect("Failed to extract files");

    Response::new(StatusCode::OK, "Files uploaded successfully")
        .data(files)
        .into()
}

pub async fn extract_files(payload: &mut Multipart) -> Result<Vec<File>, MultipartError> {
    let mut files = Vec::new();

    println!("Iterating files...");
    while let Some(mut field) = payload.try_next().await? {
        let mut data = Vec::new();

        println!("Getting file...");
        let file_name = field.name().to_string();
        println!("File name: {}", file_name);

        println!("Reading file...");
        while let Some(chunk) = field.try_next().await? {
            println!("Getting chunk: {}", chunk.len());
            data.extend_from_slice(&chunk);
        }
        println!("File read: {}", data.len());

        files.push(File {
            name: file_name,
            size: data.len(),
            data,
        });
    }

    println!("Files: {}", files.len());

    Ok(files)
}
