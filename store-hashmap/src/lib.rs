use std::collections::HashMap;

use todo_logic::{Todo, TodoStatus, TodoStore, UpdateTodo};

#[derive(Debug)]
pub enum StoreHashmapError {
    CounterError,
    NotFound,
}

#[derive(Debug)]
pub struct StoreHashmap {
    pub store: HashMap<u8, Todo>,
    counter: u8, // Note this hashmap will hold upto 255 todos. 255 because 0 will be used
}

impl StoreHashmap {
    fn new() -> StoreHashmap {
        StoreHashmap {
            store: HashMap::new(),
            counter: 0,
        }
    }
    fn load() -> StoreHashmap {
        todo!()
    }
    fn save() -> StoreHashmap {
        todo!()
    }
    fn get_counter(&mut self) -> Result<u8, StoreHashmapError> {
        if self.counter == u8::MAX {
            return Err(StoreHashmapError::CounterError);
        }

        self.counter += 1;
        Ok(self.counter)
    }
}

impl TodoStore<StoreHashmapError> for StoreHashmap {
    fn add(&mut self, name: String) -> Result<Todo, StoreHashmapError> {
        let counter = self.get_counter()?;
        let todo = Todo {
            id: counter,
            name,
            status: TodoStatus::TODO,
        };
        self.store.insert(counter, todo.clone());

        Ok(todo)
    }
    fn remove(&mut self, id: u8) -> Option<Todo> {
        self.store.remove(&id)
    }

    fn update(&mut self, todo: UpdateTodo) -> Result<Todo, StoreHashmapError> {
        // Check if the todo exists
        if let Some(old_todo) = self.get(todo.id) {
            let new_todo = Todo {
                id: todo.id,
                name: todo.name.unwrap_or(old_todo.name),
                status: todo.status.unwrap_or(old_todo.status),
            };
            self.store.insert(todo.id, new_todo.clone());
            return Ok(new_todo);
        }

        Err(StoreHashmapError::NotFound)
    }

    fn get(&self, id: u8) -> Option<Todo> {
        self.store.get(&id).cloned()
    }

    fn get_all(&self) -> Vec<Todo> {
        self.store.values().cloned().collect::<Vec<Todo>>()
    }
}

#[cfg(test)]
mod store_hashmap {

    use todo_logic::*;

    use super::StoreHashmap;

    #[test]
    fn initialize_store() {
        let hash_store = StoreHashmap::new();
        assert_eq!(hash_store.counter, 0);
        assert_eq!(hash_store.store.len(), 0);
    }

    #[test]
    fn insert() {
        let mut hash_store = StoreHashmap::new();
        let todo = hash_store.add("name".to_string()).unwrap();
        assert_eq!(todo.id, 1);
        let todo = hash_store.add("name".to_string()).unwrap();
        assert_eq!(todo.id, 2);
    }

    #[test]
    #[should_panic]
    fn insert_past_max_capacity() {
        let mut hash_store = StoreHashmap::new();

        for _ in 0..256 {
            let _ = hash_store.add("name".to_string()).unwrap();
        }
    }

    #[test]
    fn remove() {
        let mut hash_store = StoreHashmap::new();
        let todo = hash_store.add("name".to_string()).unwrap();
        assert_eq!(todo.id, 1);
        let todo = hash_store.remove(1);
        assert!(todo.is_some());
        let todo = hash_store.remove(1);
        assert!(todo.is_none())
    }

    #[test]
    #[should_panic]
    fn update_null() {
        // Update a value that isn't existing should result into an error
        let mut hash_store = StoreHashmap::new();
        hash_store
            .update(UpdateTodo {
                id: 1,
                name: None,
                status: None,
            })
            .unwrap();
    }
    #[test]
    fn update() {
        let mut hash_store = StoreHashmap::new();
        hash_store.add("name".to_string()).unwrap();

        hash_store
            .update(UpdateTodo {
                id: 1,
                name: Some("wowow".to_string()),
                status: None,
            })
            .unwrap();
        let todo = hash_store.get(1);

        match todo {
            None => panic!("Todo is missing"),
            Some(todo) => assert_eq!(todo.name, "wowow".to_string()),
        }
    }
}
