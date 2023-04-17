use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Todo {
    id: u64,
    pub text: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(id: u64, text: &str) -> Self {
        Self {
            id,
            text: text.to_string(),
            completed: false,
        }
    }
    pub fn id(&self) -> u64 {
        self.id
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct CreateTodo {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct UpdateTodo {
    pub text: Option<String>,
    pub completed: Option<bool>,
}
