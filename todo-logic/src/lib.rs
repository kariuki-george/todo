use std::cmp::PartialEq;
#[derive(Clone, Debug, PartialEq)]

pub enum TodoStatus {
    DONE,
    TODO,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Todo {
    pub id: u8, // We'll create atmost 255 todos
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
