# Todo

The todo list minimal feature set includes:

Add, remove, edit todos
Mark todos as "done"
Save and load todos

## Key features

- Has a plug configurable storage backend
  1. Full pledged db with with rusqlite - coming soon
  2. In memory hashmap that saves and loads from the fs
- Has two distint user interfaces
  1. A command line interface
  2. A web version using axum - coming soon

## Limitations

- Saving the in-memory hashmap will run only on exit command. So incase of -
  premature exit [CTRL+C], new mutations will be lost.
- If the data flushed in the file from the hashmap is tampered with, there is risk of data loss.

## Architecture

- Todo-logic

  - Contains all structs, enums describing the todos
  - Contains todo methods (add, remove, etc) trait that datastore(hashmap, rustqlite) should implement.
  - Contains trait implementation for extra functionality

- Store-hashmap

  - Contains the hashmap and an id counter. Note: The counter uses a u8 since the application is single threaded and single user.
  - Contains structs, enums and implementation for Todo trait

- todo-cmd

  - Contains the cmd user interface
  - Utilizes the hashmap to store todos.
  - The application is interactive by use of an infinite loop.
