use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TodoStatus {
    DONE,
    TODO,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Todo {
    pub id: u8, //One will create atmost 255 todos
    pub name: String,
    pub status: TodoStatus,
}

pub struct UpdateTodo {
    pub id: u8,
    pub name: Option<String>,
    pub status: Option<TodoStatus>,
}

pub trait TodoStore<E> {
    fn add(&mut self, name: String) -> Result<Todo, E>;
    fn remove(&mut self, id: u8) -> Option<Todo>;
    fn update(&mut self, todo: UpdateTodo) -> Result<Todo, E>;
    fn get(&self, id: u8) -> Option<Todo>;
    fn get_all(&self) -> Vec<Todo>;
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Id: {}, Name: {}, Status: {:?}",
            self.id, self.name, self.status
        )
    }
}
