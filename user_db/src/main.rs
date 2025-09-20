use sqlx::{SqlitePool, FromRow};

// TODO: 1. Define a `User` struct that matches your database table.
// It should have `id` (i64), `name` (String), and `email` (String).
// Don't forget to derive `Debug` and `FromRow`.
#[derive(Debug, FromRow)]
struct User {
    id : i64,
    name : String,
    email : String
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePool::connect("sqlite:users.db").await?;

    // TODO: 2. Insert two new users into the `users` table.
    // Use the `sqlx::query!` macro. Remember to use `?` placeholders.
    // User 1: name="Alice", email="alice@example.com"
    // User 2: name="Bob", email="bob@example.com"
    sqlx::query!(
        "INSERT INTO users (name, email) VALUES (?, ?)",
        "Alice",
        "alice@example.com"
    )
    .execute(&pool)
    .await?;

    sqlx::query!(
        "INSERT INTO users (name, email) VALUES ($1, $2)",
        "Bob",
        "bob@example.com"
    )
    .execute(&pool)
    .await?;

    // TODO: 3. Fetch all users from the database.
    // Use the `sqlx::query_as!` macro to map them into your `User` struct.

    let user: Vec<User> = sqlx::query_as!(User, "SELECT id, name, email FROM users").fetch_all(&pool).await?;

    // TODO: 4. Print the fetched users.
    user.iter().for_each(|user| println!("{} {} {}", user.id, user.name, user.email));


    Ok(())
}