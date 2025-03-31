//! Модуль для демонстрации асинхронного программирования в Rust
//! 
//! Этот модуль показывает различные аспекты асинхронного программирования:
//! - Асинхронные функции и блоки
//! - Фьючеры и стримы
//! - Синхронизация и каналы
//! - Таймауты и отмена
//! - Параллельное выполнение

use std::time::Duration;
use tokio::time::{sleep, timeout};
use tokio::sync::{mpsc, Mutex};
use tokio::stream::{Stream, StreamExt};
use futures::stream::FuturesUnordered;
use std::pin::Pin;

/// Структура для демонстрации асинхронных операций
#[derive(Debug)]
pub struct AsyncDemo {
    counter: Mutex<i32>,
}

/// Структура для демонстрации стримов
#[derive(Debug)]
pub struct StreamDemo {
    items: Vec<String>,
}

/// Структура для демонстрации каналов
#[derive(Debug)]
pub struct ChannelDemo {
    sender: mpsc::Sender<String>,
    receiver: mpsc::Receiver<String>,
}

impl AsyncDemo {
    /// Создание нового экземпляра
    pub fn new() -> Self {
        Self {
            counter: Mutex::new(0),
        }
    }

    /// Асинхронное увеличение счетчика
    pub async fn increment(&self) {
        let mut counter = self.counter.lock().await;
        *counter += 1;
        println!("Счетчик увеличен: {}", *counter);
    }

    /// Асинхронное получение значения счетчика
    pub async fn get_value(&self) -> i32 {
        let counter = self.counter.lock().await;
        *counter
    }
}

impl StreamDemo {
    /// Создание нового экземпляра
    pub fn new(items: Vec<String>) -> Self {
        Self { items }
    }

    /// Создание стрима из элементов
    pub fn stream(&self) -> impl Stream<Item = String> {
        tokio::stream::iter(self.items.clone())
    }

    /// Асинхронная обработка стрима
    pub async fn process_stream(&self) {
        let mut stream = self.stream();
        while let Some(item) = stream.next().await {
            println!("Обработка элемента: {}", item);
            sleep(Duration::from_millis(100)).await;
        }
    }
}

impl ChannelDemo {
    /// Создание нового экземпляра
    pub fn new(channel_size: usize) -> Self {
        let (sender, receiver) = mpsc::channel(channel_size);
        Self { sender, receiver }
    }

    /// Отправка сообщения
    pub async fn send(&self, message: String) -> Result<(), mpsc::error::SendError<String>> {
        self.sender.send(message).await
    }

    /// Получение сообщения
    pub async fn receive(&mut self) -> Option<String> {
        self.receiver.recv().await
    }
}

/// Асинхронная функция с таймаутом
pub async fn with_timeout<F, T>(future: F, duration: Duration) -> Result<T, tokio::time::error::Elapsed>
where
    F: std::future::Future<Output = T>,
{
    timeout(duration, future).await
}

/// Параллельное выполнение фьючеров
pub async fn parallel_execution() {
    let mut futures = FuturesUnordered::new();
    
    // Добавление фьючеров
    futures.push(async {
        sleep(Duration::from_millis(100)).await;
        println!("Фьючер 1 выполнен");
    });
    
    futures.push(async {
        sleep(Duration::from_millis(200)).await;
        println!("Фьючер 2 выполнен");
    });
    
    futures.push(async {
        sleep(Duration::from_millis(300)).await;
        println!("Фьючер 3 выполнен");
    });
    
    // Ожидание завершения всех фьючеров
    while let Some(_) = futures.next().await {}
}

/// Демонстрация асинхронного программирования
pub async fn demonstrate_async() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Демонстрация асинхронного программирования ===");

    // Демонстрация асинхронных операций
    println!("\n1. Асинхронные операции:");
    let demo = AsyncDemo::new();
    demo.increment().await;
    demo.increment().await;
    println!("Текущее значение: {}", demo.get_value().await);

    // Демонстрация стримов
    println!("\n2. Стримы:");
    let stream_demo = StreamDemo::new(vec![
        "Item 1".to_string(),
        "Item 2".to_string(),
        "Item 3".to_string(),
    ]);
    stream_demo.process_stream().await;

    // Демонстрация каналов
    println!("\n3. Каналы:");
    let mut channel_demo = ChannelDemo::new(10);
    channel_demo.send("Hello, Channel!".to_string()).await?;
    if let Some(message) = channel_demo.receive().await {
        println!("Получено сообщение: {}", message);
    }

    // Демонстрация таймаутов
    println!("\n4. Таймауты:");
    let result = with_timeout(
        async {
            sleep(Duration::from_millis(500)).await;
            "Успешно выполнено"
        },
        Duration::from_millis(1000),
    )
    .await;
    println!("Результат с таймаутом: {:?}", result);

    // Демонстрация параллельного выполнения
    println!("\n5. Параллельное выполнение:");
    parallel_execution().await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_async_demo() {
        let demo = AsyncDemo::new();
        demo.increment().await;
        assert_eq!(demo.get_value().await, 1);
    }

    #[test]
    async fn test_stream_demo() {
        let items = vec!["test1".to_string(), "test2".to_string()];
        let demo = StreamDemo::new(items);
        let mut stream = demo.stream();
        assert_eq!(stream.next().await, Some("test1".to_string()));
        assert_eq!(stream.next().await, Some("test2".to_string()));
        assert_eq!(stream.next().await, None);
    }

    #[test]
    async fn test_channel_demo() {
        let mut demo = ChannelDemo::new(1);
        demo.send("test".to_string()).await.unwrap();
        assert_eq!(demo.receive().await, Some("test".to_string()));
    }

    #[test]
    async fn test_timeout() {
        let result = with_timeout(
            async {
                sleep(Duration::from_millis(100)).await;
                "success"
            },
            Duration::from_millis(1000),
        )
        .await;
        assert!(result.is_ok());
    }
} 