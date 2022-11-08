use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct File {
    pub name: String,
    pub size: usize,
    #[serde(skip_serializing)]
    pub data: Vec<u8>,
}

impl File {
    pub fn new(name: String, size: usize) -> Self {
        Self {
            name,
            size,
            data: Vec::new(),
        }
    }

    pub fn from_bytes(name: String, data: Vec<u8>) -> Self {
        Self {
            name,
            size: data.len(),
            data,
        }
    }
}
