use std::process;

use dialoguer::{theme::ColorfulTheme, Select};
use store_hashmap::*;
use todo_logic::*;

#[derive(Debug)]
struct CommandsDocs {
    name: String,
    desc: String,
}

pub struct Cmd {
    db: StoreHashmap,
}

impl Cmd {
    pub fn new() -> Cmd {
        Cmd {
            db: StoreHashmap::default(),
        }
    }

    pub fn parse_commands(&mut self, input: String) {
        let commands: Vec<&str> = input.split(' ').collect();

        match commands[0].to_lowercase().as_str() {
            "exit" => self.flush(),
            "help" => self.help(),
            "list" => self.get_all(),
            "get" => {
                if commands.len() != 2 {
                    eprintln!("Incorrect usage: get todo_id");
                    return;
                }
                let id = match parse_id(commands[1].to_string()) {
                    Some(id) => id,
                    None => return,
                };
                self.get(id);
            }
            "delete" => {
                if commands.len() != 2 {
                    eprintln!("Incorrect usage: delete todo_id");
                    return;
                }
                let id = match parse_id(commands[1].to_string()) {
                    Some(id) => id,
                    None => return,
                };
                self.remove(id);
            }
            "add" => self.add(commands[1..].iter().map(|&s| s.to_string()).collect()),
            "rename" => {
                if commands.len() != 3 {
                    eprintln!("Incorrect usage: rename todo_id new_name");
                    return;
                }
                let id = match parse_id(commands[1].to_string()) {
                    Some(id) => id,
                    None => return,
                };

                let new_name = commands[2].to_string();
                self.rename(id, new_name);
            }
            "mark" => {
                if commands.len() != 3 {
                    eprintln!("Incorrect usage: rename todo_id status");
                    return;
                }
                let id = match parse_id(commands[1].to_string()) {
                    Some(id) => id,
                    None => return,
                };

                let status = parse_status(commands[2]);
                let status = match status {
                    Some(status) => status,
                    None => return,
                };

                self.mark_todo(id, status);
            }
            _ => eprintln!("Incorrect Usage or Unknown command: [command] [values]"),
        }
    }

    pub fn help(&self) {
        let commands_doc: Vec<CommandsDocs> = vec![
            CommandsDocs {
                desc: "Lists all commands - [list]".to_string(),
                name: "list".to_string(),
            },
            CommandsDocs {
                desc: "Add a new todo - [add] [name(s)]".to_string(),
                name: "add".to_string(),
            },
            CommandsDocs {
                desc: "Removed a todo - [delete] [id]".to_string(),
                name: "delete".to_string(),
            },
            CommandsDocs {
                desc: "Mark a todo as done or todo - [mark] [done or todo]".to_string(),
                name: "mark".to_string(),
            },
            CommandsDocs {
                desc: "Rename a new todo - [rename] [id] [new_nane]".to_string(),
                name: "rename".to_string(),
            },
            CommandsDocs {
                desc: "Exit program - [exit]".to_string(),
                name: "exit".to_string(),
            },
            CommandsDocs {
                desc: "Get help program - [help]".to_string(),
                name: "help".to_string(),
            },
            CommandsDocs {
                desc: "Get a todo - [get] [id]".to_string(),
                name: "get".to_string(),
            },
        ];
        println!();
        println!("Available Commands");

        println!("Command{:<3}| Description", "");
        println!();
        for command_doc in commands_doc {
            print!("{:<10}| ", command_doc.name);
            println!("{}", command_doc.desc);
        }
        println!();
    }

    fn add(&mut self, input: Vec<String>) {
        for todo_name in input {
            let todo_result = self.db.add(todo_name);
            match todo_result {
                Ok(todo) => println!("Added: {:?}", todo),
                Err(_) => eprint!("Could not add todo, Storage full"),
            }
        }
    }
    fn get_all(&self) {
        let todos = self.db.get_all();
        if todos.is_empty() {
            println!("You have no todos");
            return;
        }
        Select::with_theme(&ColorfulTheme::default())
            .default(0)
            .items(&todos[..])
            .interact()
            .unwrap();
    }
    fn get(&self, id: u8) {
        if let Some(todo) = self.db.get(id) {
            println!("{:?}", todo)
        }
    }

    fn rename(&mut self, id: u8, name: String) {
        let todo = self.db.update(UpdateTodo {
            id,
            name: Some(name),
            status: None,
        });
        match todo {
            Ok(todo) => println!("Updated: {:?}", todo),
            Err(_) => println!("Todo not found"),
        }
    }
    fn mark_todo(&mut self, id: u8, status: TodoStatus) {
        let todo = self.db.update(UpdateTodo {
            id,
            name: None,
            status: Some(status),
        });
        match todo {
            Ok(todo) => println!("Updated: {:?}", todo),
            Err(_) => println!("Todo not found"),
        }
    }
    fn remove(&mut self, id: u8) {
        self.db.remove(id);
        println!("Deleted");
    }

    pub fn flush(&mut self) {
        let result = self.db.save();
        match result {
            Ok(_) => {
                println!("Saved todos successfully!");
                process::exit(0);
            }
            Err(e) => {
                println!("Could not save the file: {:?}", e)
            }
        }
    }
}

fn parse_id(value: String) -> Option<u8> {
    let id = value.parse::<u8>();
    match id {
        Ok(id) => Some(id),
        Err(_) => {
            eprintln!("Invalid id. Id should be an integer from 1 - 255");
            None
        }
    }
}

fn parse_status(value: &str) -> Option<TodoStatus> {
    match value.to_uppercase().as_str() {
        "DONE" => Some(TodoStatus::DONE),
        "TODO" => Some(TodoStatus::TODO),
        v => {
            eprintln!(
                "Invalid todo status '{}'. Only 'todo' and 'done' are allowed",
                v
            );
            None
        }
    }
}
