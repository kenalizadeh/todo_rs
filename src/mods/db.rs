use crate::mods::todo::Todo;
use sqlx::error::Error;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Executor, Row, SqlitePool};
use std::ops::Deref;
use std::path::PathBuf;

const DB_FILENAME: &str = "todo.db";

pub async fn migrations() -> Result<(), Error> {
    let pool = conn_pool().await?;
    let query = "\
CREATE TABLE if not exists \"todo\" (
    \"id\" INTEGER NOT NULL UNIQUE,
    \"content\" TEXT NOT NULL,
    \"done\" BOOLEAN NOT NULL DEFAULT false,
    \"created_at\" DATETIME DEFAULT CURRENT_TIMESTAMP,
    \"done_at\" DATETIME DEFAULT NULL,
    PRIMARY KEY(\"id\" AUTOINCREMENT)
);
    "
    .to_string();
    pool.execute(query.as_str()).await?;
    pool.close().await;

    Ok(())
}

pub async fn add(s: &str) -> Result<(), Error> {
    let conn = conn_pool().await?;
    let query = format!("INSERT INTO \"todo\" (content) VALUES ('{}')", s);
    conn.execute(query.as_str()).await?;
    conn.close().await;

    Ok(())
}

pub async fn list(no_done: bool) -> Result<Vec<Todo>, Error> {
    let pool = conn_pool().await?;
    let mut query = "SELECT * FROM \"todo\"".to_owned();
    if no_done {
        query += "WHERE NOT DONE";
    }
    let res = sqlx::query(query.deref()).fetch_all(&pool).await?;

    pool.close().await;

    let items = res
        .iter()
        .map(|r| {
            Todo::new(
                r.get("id"),
                r.get("content"),
                r.get("done"),
                r.get("created_at"),
                r.get("done_at"),
            )
        })
        .collect();

    Ok(items)
}

pub async fn update(id: usize, content: &str) -> Result<(), Error> {
    let pool = conn_pool().await?;
    let query = format!(
        "UPDATE \"todo\" SET content = '{}' WHERE id = {}",
        content, id
    );
    pool.execute(query.as_str()).await?;
    pool.close().await;

    Ok(())
}

pub async fn delete(id: usize) -> Result<(), Error> {
    let pool = conn_pool().await?;
    let query = format!("DELETE FROM \"todo\" WHERE id = {}", id);
    pool.execute(query.as_str()).await?;
    pool.close().await;

    Ok(())
}

pub async fn toggle_done(id: usize) -> Result<(), Error> {
    let pool = conn_pool().await?;
    let query = format!(
        "UPDATE \"todo\" SET done = NOT done, done_at = case when NOT done then DATETIME('now') else NULL END WHERE id = {}",
        id
    );
    pool.execute(query.as_str()).await?;
    pool.close().await;

    Ok(())
}

async fn conn_pool() -> Result<SqlitePool, Error> {
    let mut path = home::home_dir().unwrap();
    path.push(PathBuf::from(DB_FILENAME));

    let options = SqliteConnectOptions::new()
        .filename(path.as_path())
        .create_if_missing(true);

    SqlitePool::connect_with(options).await
}
