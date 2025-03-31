//! Модуль для демонстрации асинхронного программирования в Rust
//! 
//! Этот модуль показывает различные аспекты асинхронного программирования:
//! - Асинхронные функции
//! - Фьючеры
//! - Стримы
//! - Токио для асинхронного выполнения

use tokio::time::{sleep, Duration};
use tokio::stream::{self, StreamExt};
use std::pin::Pin;
use std::future::Future;

/// Асинхронная функция для демонстрации задержки
pub async fn delay_example() {
    println!("Начало задержки");
    sleep(Duration::from_secs(2)).await;
    println!("Задержка завершена");
}

/// Асинхронная функция для демонстрации параллельного выполнения
pub async fn parallel_example() {
    let future1 = async {
        println!("Задача 1 началась");
        sleep(Duration::from_secs(1)).await;
        println!("Задача 1 завершена");
    };

    let future2 = async {
        println!("Задача 2 началась");
        sleep(Duration::from_secs(1)).await;
        println!("Задача 2 завершена");
    };

    tokio::join!(future1, future2);
}

/// Асинхронная функция для демонстрации стримов
pub async fn stream_example() {
    let mut stream = stream::iter(0..5);
    while let Some(n) = stream.next().await {
        println!("Получено число: {}", n);
        sleep(Duration::from_millis(500)).await;
    }
}

/// Структура для демонстрации асинхронных методов
#[derive(Debug)]
pub struct AsyncProcessor {
    value: i32,
}

impl AsyncProcessor {
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    /// Асинхронный метод для обработки значения
    pub async fn process(&self) -> i32 {
        sleep(Duration::from_secs(1)).await;
        self.value * 2
    }

    /// Асинхронный метод для создания фьючера
    pub fn create_future(&self) -> Pin<Box<dyn Future<Output = i32> + '_>> {
        Box::pin(self.process())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test::block_on;

    #[test]
    fn test_async_processor() {
        let processor = AsyncProcessor::new(21);
        let result = block_on(processor.process());
        assert_eq!(result, 42);
    }
} 