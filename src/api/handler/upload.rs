use crate::api::utils::types::Response;
use crate::storage::models::File;
use actix_multipart::{Multipart, MultipartError};
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, Scope};
use futures::stream::TryStreamExt;

pub fn register_endpoints() -> Scope {
    Scope::new("/file").service(web::resource("/upload").route(web::post().to(handle_file_upload)))
}

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
