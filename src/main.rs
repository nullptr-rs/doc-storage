use std::error::Error;
use std::path::PathBuf;
use std::time::SystemTime;
use doc_storage::storage::chunk;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    println!("Start benchmark...");
    let start = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();

    chunk::split(PathBuf::from("Camp 2021 Troupe Vè Versailles.mp4"), PathBuf::from("Camp 2021 Troupe Vè Versailles")).await.unwrap();

    println!();
    println!("--------------------------------------------------------------------------------");
    println!();

    chunk::combine(PathBuf::from("Camp 2021 Troupe Vè Versailles").join("manifest.json"), PathBuf::from("Camp 2021 Troupe Vè Versailles - Combined.mp4")).await.unwrap();

    println!("End benchmark...");
    let end = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();

    println!("Time: {} ms", end - start);
    Ok(())

    //https://github.com/djc/edit-chunks/blob/master/src/main.rs
    //https://lib.rs/crates/bita
}