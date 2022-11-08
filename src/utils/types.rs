use serde::Serialize;

#[derive(Serialize)]
pub struct File {
    pub name: String,
    pub size: usize,
    #[serde(skip_serializing)]
    pub data: Vec<u8>,
}
