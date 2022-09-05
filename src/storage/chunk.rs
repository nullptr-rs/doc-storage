use std::error::Error;
use std::path::PathBuf;
use md5::{Md5, Digest};
use tokio::fs::{self, File};
use serde::{Serialize, Deserialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub const CHUNK_SIZE: usize = 5 * 1024 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkedFile {
    pub name: String,
    pub dir: PathBuf,
    pub chunk_count: usize,
    pub chunks: Vec<String>,
}

pub async fn split(file_path: PathBuf, destination_dir: PathBuf) -> Result<ChunkedFile, Box<dyn Error>> {
    fs::create_dir_all(&destination_dir).await?;

    let mut chunked_file = ChunkedFile::new(file_path.file_stem().unwrap().to_str().unwrap(), fs::canonicalize(&destination_dir).await?);
    let mut file = File::open(&file_path).await?;
    let file_len = file.metadata().await?.len();
    let mut offset = 0;

    loop {
        let buf_size = if file_len - offset < CHUNK_SIZE as u64 {
            (file_len - offset) as usize
        } else {
            CHUNK_SIZE
        };


        let mut data = vec![0; buf_size];
        let size = file.read(&mut data).await?;

        if size == 0 {
            break;
        }

        let mut hasher = Md5::new();
        hasher.update(&data);
        let md5 = format!("{:x}", hasher.finalize());

        let name = format!("{}-{}-{}.chunk", chunked_file.chunk_count, md5.clone(), size);
        let path = destination_dir.join(&name);

        let mut chunk_file = File::create(&path).await?;
        chunk_file.write_all(&data).await?;

        chunked_file.chunks.push(name);
        chunked_file.chunk_count += 1;
        offset += size as u64;
    }

    let mut manifest_file = File::create(destination_dir.join("manifest.json")).await?;
    manifest_file.write_all(serde_json::to_string(&chunked_file)?.as_bytes()).await?;

    println!("Successfully chunked file '{}' ({}) into {} chunks", &chunked_file.name, &chunked_file.dir.to_str().unwrap(), &chunked_file.chunk_count);
    Ok(chunked_file.clone())
}

pub async fn combine(manifest_path: PathBuf, destination_file: PathBuf) -> Result<(ChunkedFile, fs::File), Box<dyn Error>> {
    // Manifest file should be converted to standard library file, because a Tokio file can't be read by serde_json
    let manifest_file = File::open(&manifest_path).await?;
    let chunked_file: ChunkedFile = serde_json::from_reader(manifest_file.into_std().await)?;

    fs::create_dir_all(&destination_file.parent().unwrap()).await?;
    let mut file = File::create(&destination_file).await?;
    let mut combined_chunks = 0;

    for chunk in &chunked_file.chunks {
        let chunk_path = manifest_path.parent().unwrap().join(chunk);
        let chunk_name = chunk_path.file_stem().unwrap().to_str().unwrap();
        let mut chunk_file = File::open(&chunk_path).await?;
        let mut data = Vec::new();

        let chunk_info: Vec<&str> = chunk_name.split('-').collect();

        if chunk_info.len() != 3 {
            return Err("Invalid chunk name".into());
        }

        let chunk_index = chunk_info[0].parse::<usize>()?;
        let chunk_size = chunk_info[2].parse::<usize>()?;
        let chunk_md5 = chunk_info[1];

        if chunk_index != combined_chunks {
            return Err("Chunk index does not match expected index".into());
        }

        let size = chunk_file.read_to_end(&mut data).await?;

        if size != chunk_size {
            return Err("Chunk size does not match expected size".into());
        }

        let mut hasher = Md5::new();
        hasher.update(&data);
        let md5 = format!("{:x}", hasher.finalize());

        if md5 != chunk_md5 {
            return Err("Chunk MD5 does not match expected MD5".into());
        }

        file.write_all(&data).await?;
        combined_chunks += 1;
    }

    if &combined_chunks != &chunked_file.chunk_count {
        return Err("Chunk count does not match expected chunk count".into());
    }

    println!("Successfully combined {} chunks into file '{}'", &chunked_file.chunk_count, &chunked_file.name);
    Ok((chunked_file, file))
}

impl ChunkedFile {
    pub fn new(name: &str, dir: PathBuf) -> ChunkedFile {
        ChunkedFile {
            name: name.to_string(),
            dir,
            chunk_count: 0,
            chunks: Vec::new(),
        }
    }
}