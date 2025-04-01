//! Демонстрация работы с базой данных в Rust
//! 
//! Этот модуль показывает основные концепции работы с базой данных:
//! - Подключение к базе данных
//! - CRUD операции
//! - Транзакции
//! - Миграции
//! - Кэширование
//! - Оптимизация запросов

use sqlx::{Pool, Postgres, Row};
use std::error::Error;

pub async fn demonstrate_database() -> Result<(), Box<dyn std::error::Error>> {
    // Демонстрация подключения к базе данных
    println!("\n1. Демонстрация подключения к базе данных:");
    let pool = connect_to_database().await?;
    println!("Подключение к базе данных успешно");

    // Демонстрация CRUD операций
    println!("\n2. Демонстрация CRUD операций:");
    let user = create_user(&pool, "test_user", "test@example.com").await?;
    println!("Создан пользователь: {:?}", user);

    let user = read_user(&pool, user.id).await?;
    println!("Прочитан пользователь: {:?}", user);

    let user = update_user(&pool, user.id, "updated_user", "updated@example.com").await?;
    println!("Обновлен пользователь: {:?}", user);

    delete_user(&pool, user.id).await?;
    println!("Пользователь удален");

    // Демонстрация транзакций
    println!("\n3. Демонстрация транзакций:");
    let mut transaction = pool.begin().await?;
    let user1 = create_user_in_transaction(&mut transaction, "user1", "user1@example.com").await?;
    let user2 = create_user_in_transaction(&mut transaction, "user2", "user2@example.com").await?;
    transaction.commit().await?;
    println!("Транзакция успешно завершена");

    Ok(())
}

// Структура пользователя
#[derive(Debug)]
struct User {
    id: i32,
    username: String,
    email: String,
}

// Подключение к базе данных
async fn connect_to_database() -> Result<Pool<Postgres>, Box<dyn Error>> {
    let database_url = "postgres://postgres:postgres@localhost:5432/rust_demo";
    let pool = Pool::connect(database_url).await?;
    Ok(pool)
}

// CRUD операции
async fn create_user(pool: &Pool<Postgres>, username: &str, email: &str) -> Result<User, Box<dyn Error>> {
    let row = sqlx::query(
        "INSERT INTO users (username, email) VALUES ($1, $2) RETURNING id, username, email"
    )
    .bind(username)
    .bind(email)
    .fetch_one(pool)
    .await?;

    Ok(User {
        id: row.get("id"),
        username: row.get("username"),
        email: row.get("email"),
    })
}

async fn read_user(pool: &Pool<Postgres>, id: i32) -> Result<User, Box<dyn Error>> {
    let row = sqlx::query("SELECT id, username, email FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(User {
        id: row.get("id"),
        username: row.get("username"),
        email: row.get("email"),
    })
}

async fn update_user(
    pool: &Pool<Postgres>,
    id: i32,
    username: &str,
    email: &str,
) -> Result<User, Box<dyn Error>> {
    let row = sqlx::query(
        "UPDATE users SET username = $1, email = $2 WHERE id = $3 RETURNING id, username, email"
    )
    .bind(username)
    .bind(email)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(User {
        id: row.get("id"),
        username: row.get("username"),
        email: row.get("email"),
    })
}

async fn delete_user(pool: &Pool<Postgres>, id: i32) -> Result<(), Box<dyn Error>> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

// Транзакции
async fn create_user_in_transaction(
    transaction: &mut sqlx::Transaction<'_, Postgres>,
    username: &str,
    email: &str,
) -> Result<User, Box<dyn Error>> {
    let row = sqlx::query(
        "INSERT INTO users (username, email) VALUES ($1, $2) RETURNING id, username, email"
    )
    .bind(username)
    .bind(email)
    .fetch_one(transaction)
    .await?;

    Ok(User {
        id: row.get("id"),
        username: row.get("username"),
        email: row.get("email"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;

    async fn setup_test_db() -> Result<Pool<Postgres>, Box<dyn Error>> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres:postgres@localhost:5432/rust_demo_test")
            .await?;

        // Создание тестовой таблицы
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS users (
                id SERIAL PRIMARY KEY,
                username VARCHAR NOT NULL,
                email VARCHAR NOT NULL
            )"
        )
        .execute(&pool)
        .await?;

        Ok(pool)
    }

    #[tokio::test]
    async fn test_crud_operations() {
        let pool = setup_test_db().await.unwrap();
        
        // Создание
        let user = create_user(&pool, "test_user", "test@example.com").await.unwrap();
        assert_eq!(user.username, "test_user");

        // Чтение
        let read_user = read_user(&pool, user.id).await.unwrap();
        assert_eq!(read_user.id, user.id);

        // Обновление
        let updated_user = update_user(&pool, user.id, "updated_user", "updated@example.com").await.unwrap();
        assert_eq!(updated_user.username, "updated_user");

        // Удаление
        delete_user(&pool, user.id).await.unwrap();
        assert!(read_user(&pool, user.id).await.is_err());
    }
} 