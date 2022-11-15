use blake3::Hasher;

pub async fn hash_data(data: Vec<u8>) -> Result<Vec<u8>, anyhow::Error> {
    let mut hasher = Hasher::new();
    hasher.update_rayon(&data);

    let hash = hasher.finalize();
    Ok(hash.as_bytes().to_vec())
}