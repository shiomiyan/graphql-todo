use actix_web::{web, HttpResponse};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Object, Schema, SimpleObject};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tokio_rusqlite::Connection;

use crate::sqlite::insert_todo;

pub static TODO_ID: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));
pub static TODOS: Lazy<Mutex<Vec<Todo>>> = Lazy::new(|| Mutex::new(vec![]));

#[derive(SimpleObject, Clone)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub description: String,
}

pub struct Query;

/// ref: https://async-graphql.github.io/async-graphql/en/define_complex_object.html#object
#[Object]
impl Query {
    pub async fn total_todos(&self) -> usize {
        TODOS.lock().await.len()
    }

    pub async fn all_todos(&self) -> Vec<Todo> {
        TODOS.lock().await.clone()
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    pub async fn post_todo(&self, title: String, description: String) -> bool {
        let mut id = TODO_ID.lock().await;
        *id += 1;
        //let todo = Todo {
        //    id: *id,
        //    title,
        //    description,
        //};
        //TODOS.lock().unwrap().push(todo);
        let mut conn = Connection::open("todo.db").await.unwrap();
        insert_todo(&mut conn, *id, title, description)
            .await
            .unwrap();
        conn.close().await.expect("Failed to close connection.");
        true
    }
}

pub type TodoSchema = Schema<Query, Mutation, EmptySubscription>;

pub async fn index(schema: web::Data<TodoSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

/// Router for GraphiQL playground
pub async fn index_graphiql() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish()))
}
