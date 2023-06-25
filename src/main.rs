//! Executor
//! ref: https://github.com/async-graphql/examples/blob/master/actix-web/starwars/src/main.rs
mod todo;

use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Object, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use rusqlite::named_params;
use todo::{Todo, TODOS, TODO_ID};
use tokio_rusqlite::Connection;

struct Query;

/// ref: https://async-graphql.github.io/async-graphql/en/define_complex_object.html#object
#[Object]
impl Query {
    async fn total_todos(&self) -> usize {
        TODOS.lock().await.len()
    }

    async fn all_todos(&self) -> Vec<Todo> {
        TODOS.lock().await.clone()
    }
}

struct Mutation;

#[Object]
impl Mutation {
    async fn post_todo(&self, title: String, description: String) -> bool {
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
    let mut conn = Connection::open("todo.db").await.unwrap();
    init_db(&mut conn)
        .await
        .expect("Failed to initialize database.");
    conn.close().await.expect("Failed to close connection.");

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

async fn init_db(conn: &mut Connection) -> anyhow::Result<()> {
    conn.call(|conn| {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS todos (
                id INTEGER NOT NULL PRIMARY KEY,
                title NOT NULL,
                description TEXT NOT NULL
            );",
            (),
        )
    })
    .await
    .expect("Failed to connect to database.");
    Ok(())
}

async fn insert_todo(
    conn: &mut Connection,
    id: usize,
    title: String,
    description: String,
) -> anyhow::Result<()> {
    conn.call(move |conn| {
        conn.execute(
            "INSERT INTO todos (id, title, description) VALUES (:id, :title, :description)",
            named_params! {
                ":id": id,
                ":title": &title,
                ":description": &description,
            },
        )
    })
    .await
    .expect("Failed to connect to database.");
    Ok(())
}
