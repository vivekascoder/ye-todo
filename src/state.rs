use serde::{Deserialize, Serialize};
use yewdux::store::Store;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Clone, PartialEq, Store, Default)]
pub struct TodoState {
    pub todos: Vec<Todo>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SelectedTodo {
    pub todo: Option<Todo>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShowDialog {
    pub show: bool,
}
