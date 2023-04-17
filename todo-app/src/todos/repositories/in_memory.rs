use std::collections::HashMap;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::errors::RepositoryError;
use crate::todos::models::{CreateTodo, Todo, UpdateTodo};
use crate::todos::repositories::TodoRepository;

type TodoData = HashMap<u64, Todo>;

#[derive(Debug, Clone, Default)]
pub struct TodoRepositoryInMemory {
    store: Arc<RwLock<TodoData>>,
}

impl TodoRepositoryInMemory {
    pub fn new() -> Self {
        Self::default()
    }

    fn write_store_ref(&self) -> RwLockWriteGuard<TodoData> {
        self.store.write().unwrap()
    }
    fn read_store_ref(&self) -> RwLockReadGuard<TodoData> {
        self.store.read().unwrap()
    }
}

impl TodoRepository for TodoRepositoryInMemory {
    fn create(&self, payload: CreateTodo) -> Todo {
        let mut store = self.write_store_ref();
        let id = (store.len() as u64) + 1;
        let todo = Todo::new(id, &payload.text);
        store.insert(id, todo.clone());
        todo
    }

    fn find(&self, id: u64) -> Option<Todo> {
        let store = self.read_store_ref();
        store.get(&id).map(Todo::clone)
    }

    fn all(&self) -> Vec<Todo> {
        let store = self.read_store_ref();
        store.values().map(Todo::clone).collect()
    }

    fn update(&self, id: u64, payload: UpdateTodo) -> Result<Todo, RepositoryError> {
        let mut store = self.write_store_ref();
        let Some(todo) = store.get(&id) else {
            return Err(RepositoryError::NotFound(id));
        };
        let mut new_todo = todo.clone();
        if let Some(text) = payload.text {
            new_todo.text = text;
        }
        if let Some(completed) = payload.completed {
            new_todo.completed = completed;
        }

        store.insert(id, new_todo.clone());

        Ok(new_todo)
    }

    fn delete(&self, id: u64) -> Result<(), RepositoryError> {
        let mut store = self.write_store_ref();
        let Some(_) = store.remove(&id) else {
            return Err(RepositoryError::NotFound(id));
        };
        Ok(())
    }
}
