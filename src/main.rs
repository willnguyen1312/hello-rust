use sqlx::Row; // To access the `get` method on Row
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a new connection pool
    let pool: Pool<Sqlite> = SqlitePoolOptions::new()
        .max_connections(5)
        // .connect("sqlite::memory:")
        .connect("sqlite:my_database.db")
        .await?;

    // Create a table
    // sqlx::query("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL)")
    //     .execute(&pool)
    //     .await?;

    // Insert a user
    // sqlx::query("INSERT INTO users (name) VALUES (?)")
    //     .bind("Alice")
    //     .execute(&pool)
    //     .await?;

    // Select the user back
    let rows = sqlx::query("SELECT id, name FROM users")
        .fetch_all(&pool)
        .await?;

    for row in rows {
        let id: i64 = row.get("id");
        let name = row.try_get::<String, _>("name");

        match name {
            Ok(name) => println!("Found user: {} with id: {}", name, id),
            Err(e) => println!("Error nha: {}", e),
        }
    }

    Ok(())
}
