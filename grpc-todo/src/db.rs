use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite};

use crate::config;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Todo {
    pub id: Option<i64>,
    pub title: String,
    pub completed: bool,
}

/// This struct is used to represent the database connection pool
/// and is used to interact with the database.
/// It is a wrapper around the sqlx::SqlitePool type.
/// The `sqlx::SqlitePool` type is a connection pool for SQLite databases.
/// It is used to manage multiple connections to the database.
pub async fn init_db() -> Pool<Sqlite> {
    let config = config::Config::init();
    if !Sqlite::database_exists(&config.database_url)
        .await
        .unwrap_or(false)
    {
        println!("Creating database at {}", config.database_url);
        match Sqlite::create_database(&config.database_url).await {
            Ok(_) => println!("Database created successfully"),
            Err(error) => panic!("Failed to create database: {}", error),
        };
    } else {
        println!("Database already exists at {}", config.database_url);
    }

    /// This function is used to create a new database connection pool
    let pool = sqlx::SqlitePool::connect(&config.database_url)
        .await
        .expect("Failed to connect to the database");

    /// This function is used to run the database migrations
    /// It is used to create the database schema and tables
    /// It is a wrapper around the sqlx::migrate::run function
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    /// This function is used to run the database migrations
    let migration = std::path::Path::new(&crate_dir).join("./migrations");

    let migration_results = sqlx::migrate::Migrator::new(migration)
        .await
        .unwrap()
        .run(&pool)
        .await;

    match migration_results {
        Ok(_) => println!("Database migrations ran successfully"),
        Err(error) => panic!("Failed to run database migrations: {}", error),
    };

    pool
}

pub async fn create_todo(pool: &Pool<Sqlite>, title: &str) -> Todo {
   sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (title, completed) VALUES (?, ?) RETURNING id, title, completed",
    
    ).bind(title)
    .bind(false)
    .fetch_one(pool)
    .await
    .expect("Failed to insert todo")
}

pub async fn get_todos(pool: &Pool<Sqlite>) -> Vec<Todo> {
    sqlx::query_as::<_, Todo>(
        "SELECT * FROM todos",
    )
    .fetch_all(pool)
    .await
    .expect("Failed to fetch todos")
}

pub async fn get_todo(pool: &Pool<Sqlite>, id: i64) -> Todo {
    sqlx::query_as::<_, Todo>(
        "SELECT * FROM todos WHERE id = ?",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .expect("Failed to fetch todo")
}


pub async fn update_todo(pool: &Pool<Sqlite>, id: i64, title: &str, completed: bool) -> Todo {
    sqlx::query_as::<_, Todo>(
        "UPDATE todos SET title = ?, completed = ? WHERE id = ? RETURNING id, title, completed",
    )
    .bind(title)
    .bind(completed)
    .bind(id)
    .fetch_one(pool)
    .await
    .expect("Failed to update todo")
}

pub async fn delete_todo(pool: &Pool<Sqlite>, id: i64)  {
    sqlx::query_as::<_, Todo>(
        "DELETE FROM todos WHERE id = ? RETURNING id, title, completed",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .expect("Failed to delete todo");
}