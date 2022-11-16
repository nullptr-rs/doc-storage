use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegistrationPayload {
    pub username: String,
    pub password: String,
    pub device_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
    pub device_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RefreshPayload {
    pub refresh_token: String,
}
