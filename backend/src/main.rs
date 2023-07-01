//! Executor
//! ref: https://github.com/async-graphql/examples/blob/master/actix-web/starwars/src/main.rs
mod sqlite;
mod todo;

use actix_web::{guard, web, web::Data, App, HttpServer};
use async_graphql::{EmptySubscription, Schema};
use sqlite::init_db;
use tokio_rusqlite::Connection;

use crate::todo::{index, index_graphiql};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut conn = Connection::open("todo.db").await.unwrap();
    init_db(&mut conn)
        .await
        .expect("Failed to initialize database.");
    conn.close().await.expect("Failed to close connection.");

    let schema = Schema::build(todo::Query, todo::Mutation, EmptySubscription).finish();
    println!("GraphiQL IDE is Running: http://localhost:5036");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind("0.0.0.0:5036")?
    .run()
    .await
}
