//! Executor
//! ref: https://github.com/async-graphql/examples/blob/master/actix-web/starwars/src/main.rs
mod todo;

use actix_cors::Cors;
use actix_web::{
    guard,
    http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    web,
    web::Data,
    App, HttpServer,
};
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
    println!("GraphiQL IDE is Running: http://localhost:5036/playground");
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_origin("http://localhost:5036")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![CONTENT_TYPE, AUTHORIZATION, ACCEPT])
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/graphql").guard(guard::Post()).to(index))
            .service(
                web::resource("/playground")
                    .guard(guard::Get())
                    .to(index_graphiql),
            )
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
