use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegistrationPayload {
    pub username: String,
    pub password: String,
    pub device_id: String,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
    pub device_id: String,
}

#[derive(Deserialize)]
pub struct RefreshPayload {
    pub refresh_token: String,
}
