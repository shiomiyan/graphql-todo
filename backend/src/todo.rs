use actix_web::{web, HttpResponse};
use async_graphql::{
    http::GraphiQLSource, Context, EmptySubscription, Object, Schema, SimpleObject,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tokio_rusqlite::Connection;

use rusqlite::named_params;
use uuid::Uuid;

pub static TODOS: Lazy<Mutex<Vec<Todo>>> = Lazy::new(|| Mutex::new(vec![]));

#[derive(SimpleObject, Clone)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: String,
}

impl Todo {
    pub async fn create(
        conn: &Connection,
        id: String,
        title: String,
        description: String,
    ) -> anyhow::Result<()> {
        conn.call(move |conn| {
            conn.execute(
                "INSERT INTO todo (id, title, description) VALUES (:id, :title, :description)",
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

    pub async fn read_all(conn: &Connection) -> anyhow::Result<Vec<Todo>> {
        let todo = conn
            .call(move |conn| {
                let mut statement = conn.prepare("SELECT * FROM todo")?;
                let todo = statement
                    .query_map([], |row| {
                        Ok(Todo {
                            id: row.get(0)?,
                            title: row.get(1)?,
                            description: row.get(2)?,
                        })
                    })?
                    .collect::<std::result::Result<Vec<Todo>, rusqlite::Error>>()?;

                Ok(todo)
            })
            .await
            .expect("Failed to connect to database.");

        Ok(todo)
    }
}

pub struct Query;

/// ref: https://async-graphql.github.io/async-graphql/en/define_complex_object.html#object
#[Object]
impl Query {
    pub async fn total_todos(&self) -> usize {
        TODOS.lock().await.len()
    }

    pub async fn all_todos(&self, ctx: &Context<'_>) -> Vec<Todo> {
        let conn = ctx.data::<Connection>().unwrap();
        Todo::read_all(conn).await.unwrap()
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    pub async fn post_todo(&self, ctx: &Context<'_>, title: String, description: String) -> bool {
        let id = Uuid::new_v4().to_string();
        let conn = ctx.data::<Connection>().unwrap();
        Todo::create(conn, id, title, description).await.unwrap();
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
        .body(GraphiQLSource::build().endpoint("/graphql").finish()))
}
