//! Executor
//! ref: https://github.com/async-graphql/examples/blob/master/actix-web/starwars/src/main.rs
mod todo;

use actix_web::{guard, web, web::Data, App, HttpServer};
use async_graphql::{EmptySubscription, Schema};
use tokio_rusqlite::Connection;

use crate::todo::{index, index_graphiql};

const SQLITE_DB_FILE: &str = "todo.db";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_db().await.expect("Failed to initialize database.");

    let conn = Connection::open(SQLITE_DB_FILE).await.unwrap();
    let schema = Schema::build(todo::Query, todo::Mutation, EmptySubscription)
        .data(conn)
        .finish();
    println!("GraphiQL IDE is Running: http://localhost:5036");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind("0.0.0.0:5036")?
    .run()
    .await?;

    Ok(())
}

async fn init_db() -> anyhow::Result<()> {
    let conn = Connection::open(SQLITE_DB_FILE).await.unwrap();
    conn.call(|conn| {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS todo (
                id TEXT NOT NULL PRIMARY KEY,
                title NOT NULL,
                description TEXT NOT NULL
            );",
            (),
        )
    })
    .await
    .expect("Failed to connect to database.");
    conn.close().await.expect("Failed to close connection.");
    Ok(())
}
