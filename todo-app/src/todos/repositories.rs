use crate::errors::RepositoryError;
use crate::todos::models::{CreateTodo, Todo, UpdateTodo};

pub trait TodoRepository: Clone + Send + Sync + 'static {
    fn create(&self, payload: CreateTodo) -> Todo;
    fn find(&self, id: u64) -> Option<Todo>;
    fn all(&self) -> Vec<Todo>;
    fn update(&self, id: u64, payload: UpdateTodo) -> Result<Todo, RepositoryError>;
    fn delete(&self, id: u64) -> Result<(), RepositoryError>;
}

pub mod in_memory;
