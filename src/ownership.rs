//! Модуль для демонстрации системы владения в Rust
//! 
//! Этот модуль показывает различные аспекты системы владения:
//! - Правила владения в Rust
//! - Заимствование и времена жизни
//! - Семантика перемещения
//! - Оптимизация производительности
//! - Паттерны владения

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

/// Структура для демонстрации базового владения
#[derive(Debug)]
pub struct OwnershipDemo {
    value: String,
    numbers: Vec<i32>,
}

/// Структура для демонстрации заимствования
#[derive(Debug)]
pub struct BorrowingDemo<'a> {
    reference: &'a str,
    numbers: &'a [i32],
}

/// Структура для демонстрации времен жизни
#[derive(Debug)]
pub struct LifetimeDemo<'a> {
    text: &'a str,
    metadata: HashMap<String, String>,
}

/// Структура для демонстрации внутренней мутабельности
#[derive(Debug)]
pub struct InteriorMutabilityDemo {
    counter: RefCell<i32>,
    data: Rc<RefCell<Vec<String>>>,
}

impl OwnershipDemo {
    /// Создание нового экземпляра
    pub fn new() -> Self {
        Self {
            value: String::from("Hello, Ownership!"),
            numbers: vec![1, 2, 3, 4, 5],
        }
    }

    /// Демонстрация перемещения владения
    pub fn move_ownership(self) {
        println!("Перемещение владения: {:?}", self);
    }

    /// Демонстрация заимствования
    pub fn borrow(&self) {
        println!("Заимствование: {:?}", self);
    }

    /// Демонстрация мутабельного заимствования
    pub fn borrow_mut(&mut self) {
        self.value.push_str(" (modified)");
        self.numbers.push(6);
        println!("Мутабельное заимствование: {:?}", self);
    }
}

impl<'a> BorrowingDemo<'a> {
    /// Создание нового экземпляра с заимствованием
    pub fn new(text: &'a str, numbers: &'a [i32]) -> Self {
        Self {
            reference: text,
            numbers,
        }
    }

    /// Демонстрация работы с заимствованными данными
    pub fn demonstrate_borrowing(&self) {
        println!("Заимствованный текст: {}", self.reference);
        println!("Заимствованные числа: {:?}", self.numbers);
    }
}

impl<'a> LifetimeDemo<'a> {
    /// Создание нового экземпляра с временем жизни
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            metadata: HashMap::new(),
        }
    }

    /// Добавление метаданных
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Получение текста
    pub fn get_text(&self) -> &'a str {
        self.text
    }
}

impl InteriorMutabilityDemo {
    /// Создание нового экземпляра
    pub fn new() -> Self {
        Self {
            counter: RefCell::new(0),
            data: Rc::new(RefCell::new(vec![])),
        }
    }

    /// Демонстрация внутренней мутабельности
    pub fn demonstrate_interior_mutability(&self) {
        // Мутация через RefCell
        *self.counter.borrow_mut() += 1;
        
        // Мутация через Rc<RefCell>
        self.data.borrow_mut().push(String::from("New Item"));
        
        println!("Счетчик: {}", self.counter.borrow());
        println!("Данные: {:?}", self.data.borrow());
    }
}

/// Демонстрация системы владения
pub fn demonstrate_ownership() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Демонстрация системы владения ===");

    // Демонстрация базового владения
    println!("\n1. Базовое владение:");
    let demo = OwnershipDemo::new();
    demo.borrow(); // Заимствование
    demo.move_ownership(); // Перемещение владения

    // Демонстрация заимствования
    println!("\n2. Заимствование:");
    let text = "Hello, Borrowing!";
    let numbers = vec![1, 2, 3];
    let borrowing_demo = BorrowingDemo::new(text, &numbers);
    borrowing_demo.demonstrate_borrowing();

    // Демонстрация времен жизни
    println!("\n3. Времена жизни:");
    let lifetime_text = "Hello, Lifetime!";
    let mut lifetime_demo = LifetimeDemo::new(lifetime_text);
    lifetime_demo.add_metadata("key".to_string(), "value".to_string());
    println!("Текст: {}", lifetime_demo.get_text());
    println!("Метаданные: {:?}", lifetime_demo.metadata);

    // Демонстрация внутренней мутабельности
    println!("\n4. Внутренняя мутабельность:");
    let interior_demo = InteriorMutabilityDemo::new();
    interior_demo.demonstrate_interior_mutability();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ownership_demo() {
        let demo = OwnershipDemo::new();
        assert_eq!(demo.value, "Hello, Ownership!");
        assert_eq!(demo.numbers, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_borrowing_demo() {
        let text = "test";
        let numbers = vec![1, 2];
        let demo = BorrowingDemo::new(text, &numbers);
        assert_eq!(demo.reference, "test");
        assert_eq!(demo.numbers, &[1, 2]);
    }

    #[test]
    fn test_lifetime_demo() {
        let text = "test";
        let mut demo = LifetimeDemo::new(text);
        demo.add_metadata("key".to_string(), "value".to_string());
        assert_eq!(demo.get_text(), "test");
        assert_eq!(demo.metadata.get("key").unwrap(), "value");
    }

    #[test]
    fn test_interior_mutability_demo() {
        let demo = InteriorMutabilityDemo::new();
        demo.demonstrate_interior_mutability();
        assert_eq!(*demo.counter.borrow(), 1);
        assert_eq!(demo.data.borrow().len(), 1);
    }
} 