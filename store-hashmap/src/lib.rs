use home::home_dir;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Error, Read, Write};
use std::path::Path;
use todo_logic::{Todo, TodoStatus, TodoStore, UpdateTodo};

#[derive(Debug)]
pub enum StoreHashmapError {
    CounterError,
    NotFound,
    FileError,
    SerializationError,
    IoError(Error),
    DeserializationError,
}

#[derive(Debug)]
pub struct StoreHashmap {
    store: HashMap<u8, Todo>,
    counter: u8, // Note this hashmap will hold upto 255 todos. 255 because 0 will not be used
}

impl StoreHashmap {
    fn new() -> StoreHashmap {
        StoreHashmap {
            store: HashMap::new(),
            counter: 0,
        }
    }
    fn load() -> Result<StoreHashmap, StoreHashmapError> {
        let path = get_path();

        let mut file = fs::OpenOptions::new()
            .read(true)
            .open(path)
            .map_err(StoreHashmapError::IoError)?;

        let mut bytes = vec![];
        file.read_to_end(&mut bytes)
            .map_err(StoreHashmapError::IoError)?;

        let serialized_string = String::from_utf8(bytes.clone())
            .map_err(|_| StoreHashmapError::DeserializationError)?;

        let values = serde_json::from_str::<Vec<Todo>>(serialized_string.as_str())
            .map_err(|_| StoreHashmapError::DeserializationError)?;

        let counter: u8 = values.iter().fold(1, |max: u8, elem| {
            if elem.id >= max {
                return elem.id;
            }
            max
        });

        let mut hashmap = HashMap::new();

        for todo in values {
            hashmap.insert(todo.id, todo);
        }

        Ok(StoreHashmap {
            store: hashmap.to_owned(),
            counter,
        })
    }

    // Flushes the values as bytes into a file

    pub fn save(&self) -> Result<bool, StoreHashmapError> {
        let values = self.get_all();

        let json = serde_json::to_string_pretty(&values)
            .map_err(|_| StoreHashmapError::SerializationError)?;
        let path = get_path();
        create_directories(path.clone()).map_err(StoreHashmapError::IoError)?;
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .map_err(StoreHashmapError::IoError)?;

        file.write(json.as_bytes())
            .map_err(StoreHashmapError::IoError)?;
        Ok(true)
    }
    fn get_counter(&mut self) -> Result<u8, StoreHashmapError> {
        if self.counter == u8::MAX {
            return Err(StoreHashmapError::CounterError);
        }

        self.counter += 1;
        Ok(self.counter)
    }
}

impl Default for StoreHashmap {
    // Todo: Perform load else new
    fn default() -> Self {
        let store = Self::load();
        match store {
            Err(_) => Self::new(),
            Ok(store) => store,
        }
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

fn get_path() -> String {
    let home_path = match home_dir() {
        Some(path) => String::from(
            path.join(".local")
                .join("share")
                .join("todo")
                .join("store-hash.json")
                .to_string_lossy(),
        ),
        _ => String::from("tmp/todo/store-hash.json"),
    };

    home_path
}
fn create_directories(path: String) -> io::Result<()> {
    let path = Path::new(&path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}
