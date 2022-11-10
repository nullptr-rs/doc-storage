use serde::Serialize;

#[derive(Serialize)]
pub struct RegistrationResponse {
    pub username: String,
}

impl RegistrationResponse {
    pub fn new(username: String) -> Self {
        Self { username }
    }
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
}

impl LoginResponse {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        Self {
            access_token,
            refresh_token,
        }
    }
}

#[derive(Serialize)]
pub struct RefreshResponse {
    pub access_token: String,
    pub refresh_token: String,
}

impl RefreshResponse {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        Self {
            access_token,
            refresh_token,
        }
    }
}
