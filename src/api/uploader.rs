use crate::api::types::{AsyncHttpResponse, Databases, File};
use actix_multipart::{Multipart, MultipartError};
use actix_web::{web, HttpResponse};
use futures::stream::TryStreamExt;
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    response: String,
    files: Vec<File>,
}

pub async fn upload(databases: web::Data<Databases>, mut payload: Multipart) -> AsyncHttpResponse {
    println!("Uploading file...");
    let files = get_file(&mut payload).await.expect("Error getting file");

    Ok(HttpResponse::Ok().json(Response {
        response: "File was successfully uploaded".to_string(),
        files,
    }))
}

pub async fn get_file(payload: &mut Multipart) -> Result<Vec<File>, MultipartError> {
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
