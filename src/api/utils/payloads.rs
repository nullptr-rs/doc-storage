use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegistrationPayload {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}
