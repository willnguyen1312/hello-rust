use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .connect("postgresql://forever_0eby_user:mRXHPaMrh1lQlVh29I4pybTKlKncgXId@dpg-ct89fsogph6c73d488bg-a.oregon-postgres.render.com/forever_0eby")
        .await?;

    // Drop existing table
    sqlx::query("DROP TABLE IF EXISTS users")
        .execute(&pool)
        .await?;

    // Create table with SERIAL
    sqlx::query("CREATE TABLE users (id SERIAL PRIMARY KEY, name TEXT NOT NULL)")
        .execute(&pool)
        .await?;

    // Now insert will work
    sqlx::query("INSERT INTO users (name) VALUES ($1)")
        .bind("John Doe")
        .execute(&pool)
        .await?;

    Ok(())
}
