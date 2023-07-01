use rusqlite::named_params;
use tokio_rusqlite::Connection;

pub async fn init_db(conn: &mut Connection) -> anyhow::Result<()> {
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

pub async fn insert_todo(
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
