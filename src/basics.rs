//! Демонстрация базовых концепций Rust
//! 
//! Этот модуль показывает основные концепции:
//! - Переменные и типы данных
//! - Операторы
//! - Управление потоком выполнения
//! - Функции
//! - Коллекции
//! - Строки
//! - Работа с файлами
//! - Аргументы командной строки

pub fn demonstrate_basics() {
    println!("\n1. Демонстрация переменных и типов данных:");
    let x: i32 = 42;
    let y: f64 = 3.14;
    let b: bool = true;
    let c: char = 'A';
    println!("x = {}, y = {}, b = {}, c = {}", x, y, b, c);

    println!("\n2. Демонстрация операторов:");
    let sum = x + 8;
    let product = x * 2;
    println!("sum = {}, product = {}", sum, product);

    println!("\n3. Демонстрация управления потоком:");
    if x > 40 {
        println!("x больше 40");
    } else {
        println!("x меньше или равно 40");
    }

    for i in 1..=3 {
        println!("i = {}", i);
    }

    println!("\n4. Демонстрация функций:");
    let result = add(5, 3);
    println!("5 + 3 = {}", result);

    println!("\n5. Демонстрация коллекций:");
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("вектор: {:?}", vec);

    let mut map = std::collections::HashMap::new();
    map.insert("key", "value");
    println!("map: {:?}", map);

    println!("\n6. Демонстрация строк:");
    let s1 = String::from("Hello");
    let s2 = "World";
    let s3 = format!("{} {}!", s1, s2);
    println!("{}", s3);

    println!("\n7. Демонстрация работы с файлами:");
    use std::fs;
    fs::write("test.txt", "Hello, World!").unwrap();
    let content = fs::read_to_string("test.txt").unwrap();
    println!("Содержимое файла: {}", content);
    fs::remove_file("test.txt").unwrap();

    println!("\n8. Демонстрация аргументов командной строки:");
    let args: Vec<String> = std::env::args().collect();
    println!("Аргументы командной строки: {:?}", args);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }
} 