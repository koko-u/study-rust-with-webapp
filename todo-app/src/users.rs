use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct User {
    id: u64,
    username: String,
}
impl User {
    pub fn new(id: u64, username: &str) -> Self {
        Self {
            id,
            username: username.to_string(),
        }
    }
}
