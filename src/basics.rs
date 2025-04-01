//! Модуль для демонстрации базовых концепций Rust
//! 
//! Этот модуль показывает фундаментальные концепции:
//! - Переменные и типы данных
//! - Операторы и выражения
//! - Управление потоком (if/else, match, циклы)
//! - Функции и методы
//! - Модули и видимость
//! - Работа с массивами и векторами
//! - Строки и форматирование
//! - Работа с файлами
//! - Обработка аргументов командной строки

use std::fs::{self, File};
use std::io::{self, Write, Read};
use std::path::Path;
use std::env;

/// Демонстрация переменных и типов данных
pub fn demonstrate_variables() {
    println!("\n=== Переменные и типы данных ===");
    
    // Базовые типы
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let boolean: bool = true;
    let character: char = 'A';
    
    // Вывод типов
    println!("Целое число: {} (тип: {})", integer, std::mem::size_of::<i32>());
    println!("Число с плавающей точкой: {} (тип: {})", float, std::mem::size_of::<f64>());
    println!("Булево значение: {} (тип: {})", boolean, std::mem::size_of::<bool>());
    println!("Символ: {} (тип: {})", character, std::mem::size_of::<char>());
    
    // Константы
    const MAX_VALUE: u32 = 100;
    println!("Константа: {}", MAX_VALUE);
    
    // Изменяемые переменные
    let mut mutable_var = 42;
    println!("До изменения: {}", mutable_var);
    mutable_var = 43;
    println!("После изменения: {}", mutable_var);
}

/// Демонстрация операторов и выражений
pub fn demonstrate_operators() {
    println!("\n=== Операторы и выражения ===");
    
    // Арифметические операторы
    let a = 10;
    let b = 3;
    println!("Сложение: {} + {} = {}", a, b, a + b);
    println!("Вычитание: {} - {} = {}", a, b, a - b);
    println!("Умножение: {} * {} = {}", a, b, a * b);
    println!("Деление: {} / {} = {}", a, b, a / b);
    println!("Остаток: {} % {} = {}", a, b, a % b);
    
    // Логические операторы
    let x = true;
    let y = false;
    println!("И: {} && {} = {}", x, y, x && y);
    println!("ИЛИ: {} || {} = {}", x, y, x || y);
    println!("НЕ: !{} = {}", x, !x);
    
    // Операторы сравнения
    println!("Равно: {} == {} = {}", a, b, a == b);
    println!("Не равно: {} != {} = {}", a, b, a != b);
    println!("Больше: {} > {} = {}", a, b, a > b);
    println!("Меньше: {} < {} = {}", a, b, a < b);
}

/// Демонстрация управления потоком
pub fn demonstrate_control_flow() {
    println!("\n=== Управление потоком ===");
    
    // if/else
    let number = 7;
    if number < 5 {
        println!("Число меньше 5");
    } else if number < 10 {
        println!("Число между 5 и 10");
    } else {
        println!("Число больше или равно 10");
    }
    
    // match
    match number {
        0 => println!("Ноль"),
        1..=5 => println!("От 1 до 5"),
        6..=10 => println!("От 6 до 10"),
        _ => println!("Больше 10"),
    }
    
    // Циклы
    println!("\nЦиклы:");
    
    // for
    println!("Цикл for:");
    for i in 0..3 {
        println!("Итерация: {}", i);
    }
    
    // while
    println!("\nЦикл while:");
    let mut counter = 0;
    while counter < 3 {
        println!("Счетчик: {}", counter);
        counter += 1;
    }
    
    // loop
    println!("\nЦикл loop:");
    let mut count = 0;
    loop {
        if count >= 3 {
            break;
        }
        println!("Счетчик: {}", count);
        count += 1;
    }
}

/// Демонстрация функций и методов
pub fn demonstrate_functions() {
    println!("\n=== Функции и методы ===");
    
    // Простая функция
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
    
    // Функция с несколькими параметрами
    fn calculate(x: i32, y: i32, operation: &str) -> i32 {
        match operation {
            "add" => x + y,
            "subtract" => x - y,
            "multiply" => x * y,
            "divide" => x / y,
            _ => panic!("Неизвестная операция"),
        }
    }
    
    // Функция с опциональным параметром
    fn greet(name: &str, title: Option<&str>) -> String {
        match title {
            Some(t) => format!("Привет, {} {}!", t, name),
            None => format!("Привет, {}!", name),
        }
    }
    
    // Демонстрация вызова функций
    println!("Сложение: {}", add(5, 3));
    println!("Вычисление: {}", calculate(10, 5, "multiply"));
    println!("Приветствие: {}", greet("Иван", Some("господин")));
    println!("Простое приветствие: {}", greet("Петр", None));
}

/// Демонстрация работы с массивами и векторами
pub fn demonstrate_collections() {
    println!("\n=== Коллекции ===");
    
    // Массивы
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Массив: {:?}", array);
    println!("Первый элемент: {}", array[0]);
    println!("Длина массива: {}", array.len());
    
    // Векторы
    let mut vector = Vec::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);
    println!("Вектор: {:?}", vector);
    println!("Первый элемент: {}", vector[0]);
    println!("Длина вектора: {}", vector.len());
    
    // Итерация по коллекциям
    println!("\nИтерация по массиву:");
    for element in array.iter() {
        println!("Элемент: {}", element);
    }
    
    println!("\nИтерация по вектору:");
    for element in &vector {
        println!("Элемент: {}", element);
    }
}

/// Демонстрация работы со строками
pub fn demonstrate_strings() {
    println!("\n=== Строки ===");
    
    // Строковые литералы
    let string_literal = "Привет, мир!";
    println!("Строковый литерал: {}", string_literal);
    
    // String
    let mut string = String::from("Привет");
    string.push_str(", мир!");
    println!("String: {}", string);
    
    // Форматирование строк
    let name = "Иван";
    let age = 30;
    let formatted = format!("Меня зовут {} и мне {} лет", name, age);
    println!("Форматированная строка: {}", formatted);
    
    // Методы строк
    println!("Длина строки: {}", string.len());
    println!("Пустая строка: {}", string.is_empty());
    println!("Содержит 'мир': {}", string.contains("мир"));
}

/// Демонстрация работы с файлами
pub fn demonstrate_files() -> io::Result<()> {
    println!("\n=== Работа с файлами ===");
    
    // Создание файла
    let mut file = File::create("test.txt")?;
    file.write_all(b"Привет, файл!")?;
    
    // Чтение файла
    let mut contents = String::new();
    File::open("test.txt")?.read_to_string(&mut contents)?;
    println!("Содержимое файла: {}", contents);
    
    // Проверка существования файла
    let path = Path::new("test.txt");
    println!("Файл существует: {}", path.exists());
    
    // Получение метаданных
    let metadata = fs::metadata("test.txt")?;
    println!("Размер файла: {} байт", metadata.len());
    println!("Время создания: {:?}", metadata.created()?);
    
    Ok(())
}

/// Демонстрация работы с аргументами командной строки
pub fn demonstrate_cli_args() {
    println!("\n=== Аргументы командной строки ===");
    
    let args: Vec<String> = env::args().collect();
    println!("Аргументы командной строки: {:?}", args);
    
    // Обработка аргументов
    if args.len() > 1 {
        match args[1].as_str() {
            "--help" => println!("Показать справку"),
            "--version" => println!("Версия 1.0.0"),
            arg => println!("Неизвестный аргумент: {}", arg),
        }
    }
}

/// Основная функция демонстрации
pub fn demonstrate_basics() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Демонстрация базовых концепций Rust ===");
    
    demonstrate_variables();
    demonstrate_operators();
    demonstrate_control_flow();
    demonstrate_functions();
    demonstrate_collections();
    demonstrate_strings();
    demonstrate_files()?;
    demonstrate_cli_args();
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_calculate() {
        assert_eq!(calculate(5, 3, "add"), 8);
        assert_eq!(calculate(5, 3, "subtract"), 2);
        assert_eq!(calculate(5, 3, "multiply"), 15);
        assert_eq!(calculate(6, 2, "divide"), 3);
    }

    #[test]
    fn test_greet() {
        assert_eq!(greet("Иван", Some("господин")), "Привет, господин Иван!");
        assert_eq!(greet("Петр", None), "Привет, Петр!");
    }
} 