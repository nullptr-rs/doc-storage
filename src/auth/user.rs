use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
pub struct User {
    id: i64,
    name: String,
    email: String,
    password: String,

    token: uuid::Uuid,
    is_authenticated: bool,
}

impl User {
    pub fn new(name: &str, email: &str, password: &str) -> User {
        User {
            id: 0,
            name: name.to_string(),
            email: email.to_string(),
            password: password.to_string(),
            token: uuid::Uuid::new_v4(),
            is_authenticated: true,
        }
    }
}
