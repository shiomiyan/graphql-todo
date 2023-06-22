//! Executor
//! ref: https://github.com/async-graphql/examples/blob/master/actix-web/starwars/src/main.rs

use std::sync::Mutex;

use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result};
use async_graphql::{
    http::GraphiQLSource, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use once_cell::sync::Lazy;

struct Query;

/// ref: https://async-graphql.github.io/async-graphql/en/define_complex_object.html#object
#[Object]
impl Query {
    async fn total_todos(&self) -> usize {
        TODOS.lock().unwrap().len()
    }

    async fn all_todos(&self) -> Vec<Todo> {
        TODOS.lock().unwrap().clone()
    }
}

struct Mutation;

#[Object]
impl Mutation {
    async fn post_todo(&self, title: String, description: String) -> bool {
        let mut id = TODO_ID.lock().unwrap();
        *id += 1;
        let todo = Todo {
            id: *id,
            title,
            description,
        };
        TODOS.lock().unwrap().push(todo.clone());
        true
    }
}

type TodoSchema = Schema<Query, Mutation, EmptySubscription>;

async fn index(schema: web::Data<TodoSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

/// Router for GraphiQL playground
async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(Query, Mutation, EmptySubscription).finish();
    println!("GraphiQL IDE is Running: http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

static TODO_ID: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));
static TODOS: Lazy<Mutex<Vec<Todo>>> = Lazy::new(|| Mutex::new(vec![]));

#[derive(SimpleObject, Clone)]
struct Todo {
    id: usize,
    title: String,
    description: String,
}
