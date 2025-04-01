//! Демонстрация тестирования в Rust
//! 
//! Этот модуль показывает основные концепции:
//! - Модульные тесты
//! - Интеграционные тесты
//! - Тесты производительности
//! - Моки
//! - Тестирование асинхронного кода

use std::sync::atomic::{AtomicI32, Ordering};

// Структура для демонстрации
#[derive(Debug, Clone)]
struct User {
    id: i32,
    name: String,
    email: String,
}

// Трейт для демонстрации моков
trait Database {
    fn get_user(&self, id: i32) -> Option<User>;
}

// Реализация мока
struct MockDatabase {
    users: std::collections::HashMap<i32, User>,
}

impl Database for MockDatabase {
    fn get_user(&self, id: i32) -> Option<User> {
        self.users.get(&id).cloned()
    }
}

// Счетчик для демонстрации
struct Counter {
    value: AtomicI32,
}

impl Counter {
    fn new() -> Self {
        Counter {
            value: AtomicI32::new(0),
        }
    }

    fn increment(&self) {
        self.value.fetch_add(1, Ordering::SeqCst);
    }

    fn get_value(&self) -> i32 {
        self.value.load(Ordering::SeqCst)
    }
}

pub fn demonstrate_testing() {
    println!("\n1. Демонстрация счетчика:");
    let counter = Counter::new();
    counter.increment();
    counter.increment();
    counter.increment();
    println!("Значение счетчика: {}", counter.get_value());

    println!("\n2. Демонстрация пользователя:");
    let user = User {
        id: 1,
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
    };
    println!("Пользователь: {:?}", user);

    println!("\n3. Демонстрация мока базы данных:");
    let mut mock_db = MockDatabase {
        users: std::collections::HashMap::new(),
    };
    mock_db.users.insert(1, user.clone());
    if let Some(user) = mock_db.get_user(1) {
        println!("Найден пользователь: {:?}", user);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let counter = Counter::new();
        assert_eq!(counter.get_value(), 0);
        counter.increment();
        assert_eq!(counter.get_value(), 1);
        counter.increment();
        assert_eq!(counter.get_value(), 2);
    }

    #[test]
    fn test_mock_database() {
        let mut mock_db = MockDatabase {
            users: std::collections::HashMap::new(),
        };
        let user = User {
            id: 1,
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        };
        mock_db.users.insert(1, user.clone());
        assert_eq!(mock_db.get_user(1), Some(user));
        assert_eq!(mock_db.get_user(2), None);
    }

    #[test]
    fn test_user_creation() {
        let user = User {
            id: 1,
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        };
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.email, "john@example.com");
    }
} 