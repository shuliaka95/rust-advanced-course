//! Модуль для демонстрации трейтов в Rust
//! 
//! Этот модуль показывает различные аспекты трейтов:
//! - Определение и реализация трейтов
//! - Трейты по умолчанию
//! - Ассоциированные типы
//! - Трейты с ограничениями
//! - Трейты с реализациями по умолчанию

use std::fmt;
use std::ops::Add;

/// Трейт для объектов, которые можно сериализовать
pub trait Serializable {
    /// Сериализация объекта в строку
    fn serialize(&self) -> String;
    
    /// Десериализация объекта из строки
    fn deserialize(data: &str) -> Self;
}

/// Трейт для объектов с ассоциированным типом
pub trait Container {
    /// Тип элемента контейнера
    type Item;
    
    /// Добавление элемента
    fn add(&mut self, item: Self::Item);
    
    /// Получение элемента по индексу
    fn get(&self, index: usize) -> Option<&Self::Item>;
    
    /// Удаление элемента по индексу
    fn remove(&mut self, index: usize) -> Option<Self::Item>;
}

/// Трейт для объектов с реализацией по умолчанию
pub trait Printable {
    /// Форматирование объекта
    fn format(&self) -> String {
        String::from("Default format")
    }
    
    /// Вывод объекта
    fn print(&self) {
        println!("{}", self.format());
    }
}

/// Структура для демонстрации сериализации
#[derive(Debug)]
pub struct User {
    name: String,
    age: u32,
}

/// Структура для демонстрации контейнера
#[derive(Debug)]
pub struct DynamicArray<T> {
    items: Vec<T>,
}

/// Структура для демонстрации печати
#[derive(Debug)]
pub struct Document {
    content: String,
}

impl Serializable for User {
    fn serialize(&self) -> String {
        format!("{{\"name\":\"{}\",\"age\":{}}}", self.name, self.age)
    }
    
    fn deserialize(data: &str) -> Self {
        // Упрощенная реализация для демонстрации
        let parts: Vec<&str> = data.split(',').collect();
        let name = parts[0].split(':').nth(1).unwrap().trim_matches('"');
        let age = parts[1].split(':').nth(1).unwrap().parse().unwrap();
        
        Self {
            name: name.to_string(),
            age,
        }
    }
}

impl<T> Container for DynamicArray<T> {
    type Item = T;
    
    fn add(&mut self, item: T) {
        self.items.push(item);
    }
    
    fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }
    
    fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }
}

impl Printable for Document {
    fn format(&self) -> String {
        format!("Document: {}", self.content)
    }
}

/// Трейт для математических операций
pub trait MathOperations<T> {
    fn add(&self, other: &T) -> T;
    fn multiply(&self, other: &T) -> T;
}

/// Структура для демонстрации математических операций
#[derive(Debug)]
pub struct Complex {
    real: f64,
    imag: f64,
}

impl MathOperations<Complex> for Complex {
    fn add(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
    
    fn multiply(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

/// Демонстрация трейтов
pub fn demonstrate_traits() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Демонстрация трейтов ===");

    // Демонстрация сериализации
    println!("\n1. Сериализация:");
    let user = User {
        name: String::from("John"),
        age: 30,
    };
    let serialized = user.serialize();
    println!("Сериализованный пользователь: {}", serialized);
    let deserialized = User::deserialize(&serialized);
    println!("Десериализованный пользователь: {:?}", deserialized);

    // Демонстрация контейнера
    println!("\n2. Контейнер:");
    let mut array = DynamicArray { items: vec![] };
    array.add(1);
    array.add(2);
    array.add(3);
    println!("Элемент по индексу 1: {:?}", array.get(1));
    println!("Удаленный элемент: {:?}", array.remove(1));

    // Демонстрация печати
    println!("\n3. Печать:");
    let doc = Document {
        content: String::from("Hello, World!"),
    };
    doc.print();

    // Демонстрация математических операций
    println!("\n4. Математические операции:");
    let c1 = Complex { real: 1.0, imag: 2.0 };
    let c2 = Complex { real: 3.0, imag: 4.0 };
    let sum = c1.add(&c2);
    let product = c1.multiply(&c2);
    println!("Сумма: {:?}", sum);
    println!("Произведение: {:?}", product);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization() {
        let user = User {
            name: String::from("test"),
            age: 25,
        };
        let serialized = user.serialize();
        let deserialized = User::deserialize(&serialized);
        assert_eq!(deserialized.name, "test");
        assert_eq!(deserialized.age, 25);
    }

    #[test]
    fn test_container() {
        let mut array = DynamicArray { items: vec![] };
        array.add(1);
        array.add(2);
        assert_eq!(*array.get(0).unwrap(), 1);
        assert_eq!(array.remove(0).unwrap(), 1);
    }

    #[test]
    fn test_printable() {
        let doc = Document {
            content: String::from("test"),
        };
        assert_eq!(doc.format(), "Document: test");
    }

    #[test]
    fn test_math_operations() {
        let c1 = Complex { real: 1.0, imag: 2.0 };
        let c2 = Complex { real: 3.0, imag: 4.0 };
        let sum = c1.add(&c2);
        assert_eq!(sum.real, 4.0);
        assert_eq!(sum.imag, 6.0);
    }
} 