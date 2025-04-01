//! Модуль для демонстрации работы с базами данных в Rust
//! 
//! Этот модуль показывает различные аспекты работы с БД:
//! - CRUD операции
//! - Транзакции
//! - Миграции
//! - Асинхронные запросы

use sqlx::{Pool, Postgres, Row};
use sqlx::postgres::PgPoolOptions;
use std::error::Error;
use chrono::{DateTime, Utc};
use tokio::time::Duration;

/// Структура для представления пользователя
#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

/// Реализация CRUD операций для пользователей
pub struct UserRepository {
    pool: Pool<Postgres>,
}

impl UserRepository {
    /// Создание нового репозитория
    pub async fn new(database_url: &str) -> Result<Self, Box<dyn Error>> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;
        Ok(Self { pool })
    }

    /// Создание пользователя
    pub async fn create(&self, name: &str, email: &str) -> Result<User, Box<dyn Error>> {
        let row = sqlx::query!(
            r#"
            INSERT INTO users (name, email, created_at)
            VALUES ($1, $2, NOW())
            RETURNING id, name, email, created_at
            "#,
            name,
            email
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(User {
            id: row.id,
            name: row.name,
            email: row.email,
            created_at: row.created_at,
        })
    }

    /// Получение пользователя по ID
    pub async fn get_by_id(&self, id: i32) -> Result<Option<User>, Box<dyn Error>> {
        let row = sqlx::query!(
            r#"
            SELECT id, name, email, created_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|row| User {
            id: row.id,
            name: row.name,
            email: row.email,
            created_at: row.created_at,
        }))
    }

    /// Обновление пользователя
    pub async fn update(&self, id: i32, name: &str, email: &str) -> Result<User, Box<dyn Error>> {
        let row = sqlx::query!(
            r#"
            UPDATE users
            SET name = $1, email = $2
            WHERE id = $3
            RETURNING id, name, email, created_at
            "#,
            name,
            email,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(User {
            id: row.id,
            name: row.name,
            email: row.email,
            created_at: row.created_at,
        })
    }

    /// Удаление пользователя
    pub async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>> {
        sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Получение всех пользователей
    pub async fn get_all(&self) -> Result<Vec<User>, Box<dyn Error>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name, email, created_at
            FROM users
            ORDER BY id
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| User {
                id: row.id,
                name: row.name,
                email: row.email,
                created_at: row.created_at,
            })
            .collect())
    }
}

/// Демонстрация CRUD операций
pub async fn demonstrate_crud_operations() -> Result<(), Box<dyn Error>> {
    let database_url = "postgres://postgres:postgres@localhost/rust_demo";
    let repo = UserRepository::new(database_url).await?;

    // Создание пользователя
    let user = repo.create("Иван", "ivan@example.com").await?;
    println!("Создан пользователь: {:?}", user);

    // Получение пользователя
    if let Some(user) = repo.get_by_id(user.id).await? {
        println!("Получен пользователь: {:?}", user);
    }

    // Обновление пользователя
    let updated_user = repo.update(user.id, "Иван Иванов", "ivan.ivanov@example.com").await?;
    println!("Обновлен пользователь: {:?}", updated_user);

    // Получение всех пользователей
    let users = repo.get_all().await?;
    println!("Все пользователи: {:?}", users);

    // Удаление пользователя
    repo.delete(user.id).await?;
    println!("Пользователь удален");

    Ok(())
}

/// Демонстрация транзакций
pub async fn demonstrate_transactions() -> Result<(), Box<dyn Error>> {
    let database_url = "postgres://postgres:postgres@localhost/rust_demo";
    let repo = UserRepository::new(database_url).await?;

    // Начало транзакции
    let mut tx = repo.pool.begin().await?;

    // Создание пользователей в транзакции
    let user1 = sqlx::query!(
        r#"
        INSERT INTO users (name, email, created_at)
        VALUES ($1, $2, NOW())
        RETURNING id, name, email, created_at
        "#,
        "Алексей",
        "alex@example.com"
    )
    .fetch_one(&mut *tx)
    .await?;

    let user2 = sqlx::query!(
        r#"
        INSERT INTO users (name, email, created_at)
        VALUES ($1, $2, NOW())
        RETURNING id, name, email, created_at
        "#,
        "Мария",
        "maria@example.com"
    )
    .fetch_one(&mut *tx)
    .await?;

    // Подтверждение транзакции
    tx.commit().await?;

    println!("Созданы пользователи в транзакции:");
    println!("1. {:?}", user1);
    println!("2. {:?}", user2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    async fn setup_test_db() -> Result<UserRepository, Box<dyn Error>> {
        let database_url = "postgres://postgres:postgres@localhost/rust_demo_test";
        let repo = UserRepository::new(database_url).await?;
        
        // Очистка тестовой базы данных
        sqlx::query!("TRUNCATE TABLE users CASCADE")
            .execute(&repo.pool)
            .await?;
        
        Ok(repo)
    }

    #[tokio::test]
    async fn test_user_repository() -> Result<(), Box<dyn Error>> {
        let repo = setup_test_db().await?;

        // Создание пользователя
        let user = repo.create("Тест", "test@example.com").await?;
        assert_eq!(user.name, "Тест");
        assert_eq!(user.email, "test@example.com");

        // Получение пользователя
        let retrieved = repo.get_by_id(user.id).await?.unwrap();
        assert_eq!(retrieved.id, user.id);

        // Обновление пользователя
        let updated = repo.update(user.id, "Обновленный", "updated@example.com").await?;
        assert_eq!(updated.name, "Обновленный");
        assert_eq!(updated.email, "updated@example.com");

        // Удаление пользователя
        repo.delete(user.id).await?;
        assert!(repo.get_by_id(user.id).await?.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_transaction_rollback() -> Result<(), Box<dyn Error>> {
        let repo = setup_test_db().await?;
        let mut tx = repo.pool.begin().await?;

        // Создание пользователя в транзакции
        let user = sqlx::query!(
            r#"
            INSERT INTO users (name, email, created_at)
            VALUES ($1, $2, NOW())
            RETURNING id, name, email, created_at
            "#,
            "Транзакция",
            "transaction@example.com"
        )
        .fetch_one(&mut *tx)
        .await?;

        // Откат транзакции
        tx.rollback().await?;

        // Проверяем, что пользователь не был создан
        let result = sqlx::query!(
            r#"
            SELECT id FROM users WHERE id = $1
            "#,
            user.id
        )
        .fetch_optional(&repo.pool)
        .await?;

        assert!(result.is_none());

        Ok(())
    }
} 