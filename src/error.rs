//! Модуль для демонстрации обработки ошибок в Rust
//! 
//! Этот модуль показывает различные аспекты обработки ошибок:
//! - Пользовательские типы ошибок
//! - Result и Option
//! - Propagating ошибок
//! - Обработка ошибок в асинхронном коде
//! - Логирование ошибок

use std::error::Error;
use std::fmt;
use std::io;
use std::num::ParseIntError;
use thiserror::Error;

/// Пользовательский тип ошибки для демонстрации
#[derive(Debug, Error)]
pub enum CustomError {
    #[error("Ошибка ввода/вывода: {0}")]
    Io(#[from] io::Error),
    
    #[error("Ошибка парсинга числа: {0}")]
    Parse(#[from] ParseIntError),
    
    #[error("Пользовательская ошибка: {0}")]
    Custom(String),
    
    #[error("Ошибка валидации: {0}")]
    Validation(String),
}

/// Структура для демонстрации обработки ошибок
#[derive(Debug)]
pub struct ErrorDemo {
    value: i32,
}

/// Структура для демонстрации асинхронной обработки ошибок
#[derive(Debug)]
pub struct AsyncErrorDemo {
    data: Vec<String>,
}

impl ErrorDemo {
    /// Создание нового экземпляра
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    /// Демонстрация Result
    pub fn divide(&self, divisor: i32) -> Result<i32, CustomError> {
        if divisor == 0 {
            return Err(CustomError::Custom("Деление на ноль".to_string()));
        }
        Ok(self.value / divisor)
    }

    /// Демонстрация Option
    pub fn get_positive(&self) -> Option<i32> {
        if self.value > 0 {
            Some(self.value)
        } else {
            None
        }
    }

    /// Демонстрация обработки ошибок
    pub fn process_data(&self, input: &str) -> Result<i32, CustomError> {
        // Парсинг строки в число
        let number = input.parse::<i32>()?;
        
        // Проверка на положительное число
        if number <= 0 {
            return Err(CustomError::Validation("Число должно быть положительным".to_string()));
        }
        
        Ok(number)
    }
}

impl AsyncErrorDemo {
    /// Создание нового экземпляра
    pub fn new(data: Vec<String>) -> Self {
        Self { data }
    }

    /// Асинхронная обработка данных
    pub async fn process_async(&self, index: usize) -> Result<String, CustomError> {
        if index >= self.data.len() {
            return Err(CustomError::Custom("Индекс вне диапазона".to_string()));
        }
        
        // Имитация асинхронной операции
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        Ok(self.data[index].clone())
    }

    /// Асинхронная обработка нескольких элементов
    pub async fn process_multiple(&self, indices: &[usize]) -> Result<Vec<String>, CustomError> {
        let mut results = Vec::new();
        
        for &index in indices {
            match self.process_async(index).await {
                Ok(value) => results.push(value),
                Err(e) => return Err(e),
            }
        }
        
        Ok(results)
    }
}

/// Демонстрация обработки ошибок
pub fn demonstrate_error_handling() -> Result<(), Box<dyn Error>> {
    println!("\n=== Демонстрация обработки ошибок ===");

    // Демонстрация Result
    println!("\n1. Демонстрация Result:");
    let demo = ErrorDemo::new(10);
    match demo.divide(2) {
        Ok(result) => println!("Результат деления: {}", result),
        Err(e) => println!("Ошибка: {}", e),
    }

    // Демонстрация Option
    println!("\n2. Демонстрация Option:");
    let demo = ErrorDemo::new(5);
    if let Some(value) = demo.get_positive() {
        println!("Положительное число: {}", value);
    } else {
        println!("Число не положительное");
    }

    // Демонстрация обработки ошибок
    println!("\n3. Демонстрация обработки ошибок:");
    let demo = ErrorDemo::new(0);
    match demo.process_data("42") {
        Ok(result) => println!("Успешно обработано: {}", result),
        Err(e) => println!("Ошибка обработки: {}", e),
    }

    Ok(())
}

/// Демонстрация асинхронной обработки ошибок
pub async fn demonstrate_async_error_handling() -> Result<(), Box<dyn Error>> {
    println!("\n=== Демонстрация асинхронной обработки ошибок ===");

    let demo = AsyncErrorDemo::new(vec![
        "Item 1".to_string(),
        "Item 2".to_string(),
        "Item 3".to_string(),
    ]);

    // Демонстрация асинхронной обработки
    println!("\n1. Асинхронная обработка:");
    match demo.process_async(1).await {
        Ok(result) => println!("Результат: {}", result),
        Err(e) => println!("Ошибка: {}", e),
    }

    // Демонстрация обработки нескольких элементов
    println!("\n2. Обработка нескольких элементов:");
    match demo.process_multiple(&[0, 1, 2]).await {
        Ok(results) => println!("Результаты: {:?}", results),
        Err(e) => println!("Ошибка: {}", e),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_demo() {
        let demo = ErrorDemo::new(10);
        assert_eq!(demo.divide(2).unwrap(), 5);
        assert!(demo.divide(0).is_err());
        assert_eq!(demo.get_positive().unwrap(), 10);
        assert!(demo.process_data("42").is_ok());
        assert!(demo.process_data("-1").is_err());
    }

    #[tokio::test]
    async fn test_async_error_demo() {
        let demo = AsyncErrorDemo::new(vec!["test".to_string()]);
        assert_eq!(demo.process_async(0).await.unwrap(), "test");
        assert!(demo.process_async(1).await.is_err());
        assert_eq!(demo.process_multiple(&[0]).await.unwrap(), vec!["test"]);
        assert!(demo.process_multiple(&[1]).await.is_err());
    }
} 