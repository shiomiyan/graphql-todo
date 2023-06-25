use async_graphql::SimpleObject;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

pub static TODO_ID: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));
pub static TODOS: Lazy<Mutex<Vec<Todo>>> = Lazy::new(|| Mutex::new(vec![]));

#[derive(SimpleObject, Clone)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub description: String,
}
