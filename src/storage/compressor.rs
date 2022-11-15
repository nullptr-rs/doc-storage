use flate2::write::{ZlibDecoder, ZlibEncoder};
use flate2::Compression;
use std::io::Write;

pub async fn compress_data(data: Vec<u8>) -> Result<Vec<u8>, anyhow::Error> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(&data)?;

    let compressed_data = encoder.finish()?;
    Ok(compressed_data)
}

pub async fn decompress_data(data: Vec<u8>) -> Result<Vec<u8>, anyhow::Error> {
    let mut decoder = ZlibDecoder::new(Vec::new());
    decoder.write_all(&data)?;

    let decompressed_data = decoder.finish()?;
    Ok(decompressed_data)
}
