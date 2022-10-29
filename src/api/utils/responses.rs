use serde::Serialize;

#[derive(Serialize)]
pub struct RegistrationResponse {
    pub username: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}
