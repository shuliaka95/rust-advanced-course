//! Демонстрация асинхронного программирования в Rust
//! 
//! Этот модуль показывает основные концепции:
//! - Асинхронные функции
//! - Фьючеры
//! - Стримы
//! - Токио
//! - Асинхронные трейты

use tokio::time::{sleep, Duration};
use futures::stream::{self, StreamExt};
use tokio_stream::StreamExt as _;

// Асинхронная функция
async fn fetch_data(id: u32) -> String {
    sleep(Duration::from_millis(100)).await;
    format!("Данные для ID {}", id)
}

// Асинхронная функция с параллельным выполнением
async fn fetch_multiple_data() {
    let data1 = fetch_data(1);
    let data2 = fetch_data(2);
    let data3 = fetch_data(3);

    let results = tokio::join!(data1, data2, data3);
    println!("Результаты: {:?}", results);
}

// Асинхронный стрим
async fn process_stream() {
    let mut stream = stream::iter(1..=5)
        .map(|n| async move {
            sleep(Duration::from_millis(100)).await;
            n
        })
        .buffer_unordered(3);

    while let Some(n) = stream.next().await {
        println!("Получено число: {}", n);
    }
}

pub async fn demonstrate_async() {
    println!("\n1. Демонстрация асинхронных функций:");
    let result = fetch_data(1).await;
    println!("{}", result);

    println!("\n2. Демонстрация параллельного выполнения:");
    fetch_multiple_data().await;

    println!("\n3. Демонстрация асинхронного стрима:");
    process_stream().await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_fetch_data() {
        let result = fetch_data(1).await;
        assert_eq!(result, "Данные для ID 1");
    }
} 