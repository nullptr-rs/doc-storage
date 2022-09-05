use doc_storage::storage::chunk;
use std::error::Error;
use std::path::PathBuf;
use std::time::SystemTime;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Start benchmark...");
    let start_split = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    chunk::split(
        PathBuf::from("Open Ocean 10 Hours of Relaxing Oceanscapes  BBC Earth.mp4"),
        PathBuf::from("Open Ocean 10 Hours of Relaxing Oceanscapes  BBC Earth"),
    )
    .await
    .unwrap();

    println!(
        "Finished in {}ms",
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            - start_split
    );
    println!("--------------------------------------------------------------------------------");
    println!("Start benchmark...");

    let start_combine = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    chunk::combine(
        PathBuf::from("Open Ocean 10 Hours of Relaxing Oceanscapes  BBC Earth")
            .join("manifest.json"),
        PathBuf::from("Open Ocean 10 Hours of Relaxing Oceanscapes  BBC Earth - Combined.mp4"),
    )
    .await
    .unwrap();

    println!(
        "Finished in {}ms",
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            - start_combine
    );
    Ok(())

    //https://lib.rs/crates/bita
    //https://discord.com/channels/648981252988338213/935847071540469820/1016443689679200286
}
