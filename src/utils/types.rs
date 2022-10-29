use serde::Serialize;
use std::collections::HashMap;

pub type SessionState = HashMap<String, String>;

#[derive(Serialize)]
pub struct File {
    pub name: String,
    pub size: usize,
    #[serde(skip_serializing)]
    pub data: Vec<u8>,
}
