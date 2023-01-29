use crate::state::Todo;
use gloo_storage::{LocalStorage, Storage};
use std::error::Error;

pub fn get_todos_from_localstorage() -> Vec<Todo> {
    let local_storage_data: Vec<Todo> = match LocalStorage::get("todos") {
        Ok(data) => data,
        Err(_) => {
            LocalStorage::set("todos", "[]").unwrap();
            vec![]
        }
    };

    println!("Got: {:?}", local_storage_data);
    local_storage_data
}

pub fn insert_todo_to_localstorage(todo: Todo) -> Result<(), Box<dyn Error>> {
    let todos = get_todos_from_localstorage();
    let new_todos: Vec<Todo> = todos.iter().cloned().chain(Some(todo)).collect();
    LocalStorage::set("todos", &new_todos)?;
    Ok(())
}

pub fn delete_todo_from_localstorage(todo_id: i32) -> Result<(), Box<dyn Error>> {
    let todos = get_todos_from_localstorage();
    let new_todos: Vec<Todo> = todos.iter().cloned().filter(|t| t.id != todo_id).collect();
    LocalStorage::set("todos", &new_todos)?;
    Ok(())
}

pub fn update_todo_title(todo_id: i32, title: String) -> Result<(), Box<dyn Error>> {
    let todos = get_todos_from_localstorage();
    let new_todos: Vec<Todo> = todos
        .iter()
        .cloned()
        .map(|t| {
            if t.id == todo_id {
                Todo {
                    id: t.id,
                    title: title.clone(),
                    completed: t.completed,
                }
            } else {
                t
            }
        })
        .collect();
    LocalStorage::set("todos", &new_todos)?;
    Ok(())
}

pub fn mark_todo_from_localstorage(todo_id: i32, state: bool) -> Result<(), Box<dyn Error>> {
    let todos = get_todos_from_localstorage();
    let new_todos: Vec<Todo> = todos
        .iter()
        .cloned()
        .map(|t| {
            if t.id == todo_id {
                Todo {
                    id: t.id,
                    title: t.title,
                    completed: state,
                }
            } else {
                t
            }
        })
        .collect();
    LocalStorage::set("todos", &new_todos)?;
    Ok(())
}
