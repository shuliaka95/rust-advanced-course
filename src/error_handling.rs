//! Демонстрация обработки ошибок в Rust
//! 
//! Этот модуль показывает основные концепции обработки ошибок:
//! - Result и Option
//! - Создание пользовательских ошибок
//! - Распространение ошибок
//! - Обработка ошибок с контекстом
//! - Комбинирование ошибок
//! - Обработка паники

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

pub fn demonstrate_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    // Демонстрация Result и Option
    println!("\n1. Демонстрация Result и Option:");
    match divide(10, 2) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Ошибка: {}", e),
    }

    // Демонстрация пользовательских ошибок
    println!("\n2. Демонстрация пользовательских ошибок:");
    match process_data("invalid") {
        Ok(_) => println!("Данные обработаны успешно"),
        Err(e) => println!("Ошибка обработки: {}", e),
    }

    // Демонстрация распространения ошибок
    println!("\n3. Демонстрация распространения ошибок:");
    match parse_and_divide("10", "2") {
        Ok(result) => println!("Результат: {}", result),
        Err(e) => println!("Ошибка: {}", e),
    }

    // Демонстрация обработки ошибок с контекстом
    println!("\n4. Демонстрация обработки ошибок с контекстом:");
    match read_file("nonexistent.txt") {
        Ok(_) => println!("Файл прочитан успешно"),
        Err(e) => println!("Ошибка чтения файла: {}", e),
    }

    Ok(())
}

// Пользовательский тип ошибки
#[derive(Debug)]
pub enum CustomError {
    InvalidInput(String),
    ProcessingError(String),
    IoError(std::io::Error),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::InvalidInput(msg) => write!(f, "Неверный ввод: {}", msg),
            CustomError::ProcessingError(msg) => write!(f, "Ошибка обработки: {}", msg),
            CustomError::IoError(e) => write!(f, "Ошибка ввода-вывода: {}", e),
        }
    }
}

impl Error for CustomError {}

// Функции для демонстрации
fn divide(a: i32, b: i32) -> Result<i32, CustomError> {
    if b == 0 {
        Err(CustomError::InvalidInput("Деление на ноль".to_string()))
    } else {
        Ok(a / b)
    }
}

fn process_data(data: &str) -> Result<(), CustomError> {
    if data == "invalid" {
        Err(CustomError::InvalidInput("Неверные данные".to_string()))
    } else {
        Ok(())
    }
}

fn parse_and_divide(a: &str, b: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let a_num = a.parse::<i32>()?;
    let b_num = b.parse::<i32>()?;
    Ok(a_num / b_num)
}

fn read_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    std::fs::read_to_string(path).map_err(|e| CustomError::IoError(e).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2).unwrap(), 5);
        assert!(divide(10, 0).is_err());
    }

    #[test]
    fn test_process_data() {
        assert!(process_data("valid").is_ok());
        assert!(process_data("invalid").is_err());
    }

    #[test]
    fn test_parse_and_divide() {
        assert_eq!(parse_and_divide("10", "2").unwrap(), 5);
        assert!(parse_and_divide("10", "0").is_err());
    }
} 