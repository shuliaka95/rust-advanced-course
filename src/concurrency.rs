//! Модуль для демонстрации конкурентного программирования в Rust
//! 
//! Этот модуль показывает различные аспекты конкурентного программирования:
//! - Потоки и мьютексы
//! - Каналы и сообщения
//! - Атомарные операции
//! - Синхронизация
//! - Параллельное выполнение

use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;
use tokio::sync::{mpsc, Semaphore};
use tokio::time::sleep;
use std::sync::atomic::{AtomicI32, Ordering};

/// Структура для демонстрации потоков
#[derive(Debug)]
pub struct ThreadDemo {
    counter: Arc<Mutex<i32>>,
    condition: Arc<Condvar>,
}

/// Структура для демонстрации каналов
#[derive(Debug)]
pub struct ChannelDemo {
    sender: mpsc::Sender<String>,
    receiver: mpsc::Receiver<String>,
}

/// Структура для демонстрации атомарных операций
#[derive(Debug)]
pub struct AtomicDemo {
    counter: Arc<AtomicI32>,
}

/// Структура для демонстрации синхронизации
#[derive(Debug)]
pub struct SyncDemo {
    semaphore: Arc<Semaphore>,
    data: Arc<Mutex<Vec<String>>>,
}

impl ThreadDemo {
    /// Создание нового экземпляра
    pub fn new() -> Self {
        Self {
            counter: Arc::new(Mutex::new(0)),
            condition: Arc::new(Condvar::new()),
        }
    }

    /// Демонстрация работы с потоками
    pub fn demonstrate_threads(&self) {
        let counter = Arc::clone(&self.counter);
        let condition = Arc::clone(&self.condition);

        // Создание потока для увеличения счетчика
        let increment_thread = thread::spawn(move || {
            for _ in 0..5 {
                let mut counter = counter.lock().unwrap();
                *counter += 1;
                println!("Счетчик увеличен: {}", *counter);
                condition.notify_one();
                thread::sleep(Duration::from_millis(100));
            }
        });

        // Создание потока для ожидания изменений
        let wait_thread = thread::spawn(move || {
            let mut counter = counter.lock().unwrap();
            while *counter < 5 {
                counter = condition.wait(counter).unwrap();
                println!("Получено уведомление: {}", *counter);
            }
        });

        // Ожидание завершения потоков
        increment_thread.join().unwrap();
        wait_thread.join().unwrap();
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

impl AtomicDemo {
    /// Создание нового экземпляра
    pub fn new() -> Self {
        Self {
            counter: Arc::new(AtomicI32::new(0)),
        }
    }

    /// Увеличение счетчика
    pub fn increment(&self) {
        self.counter.fetch_add(1, Ordering::SeqCst);
        println!("Атомарный счетчик: {}", self.counter.load(Ordering::SeqCst));
    }

    /// Получение значения счетчика
    pub fn get_value(&self) -> i32 {
        self.counter.load(Ordering::SeqCst)
    }
}

impl SyncDemo {
    /// Создание нового экземпляра
    pub fn new(permits: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(permits)),
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Добавление данных с синхронизацией
    pub async fn add_data(&self, item: String) {
        let _permit = self.semaphore.acquire().await.unwrap();
        let mut data = self.data.lock().unwrap();
        data.push(item);
        println!("Данные добавлены: {:?}", data);
    }

    /// Получение данных
    pub fn get_data(&self) -> Vec<String> {
        self.data.lock().unwrap().clone()
    }
}

/// Демонстрация конкурентного программирования
pub async fn demonstrate_concurrency() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Демонстрация конкурентного программирования ===");

    // Демонстрация потоков
    println!("\n1. Демонстрация потоков:");
    let thread_demo = ThreadDemo::new();
    thread_demo.demonstrate_threads();

    // Демонстрация каналов
    println!("\n2. Демонстрация каналов:");
    let mut channel_demo = ChannelDemo::new(10);
    channel_demo.send("Hello, Channel!".to_string()).await?;
    if let Some(message) = channel_demo.receive().await {
        println!("Получено сообщение: {}", message);
    }

    // Демонстрация атомарных операций
    println!("\n3. Демонстрация атомарных операций:");
    let atomic_demo = AtomicDemo::new();
    atomic_demo.increment();
    atomic_demo.increment();
    println!("Финальное значение: {}", atomic_demo.get_value());

    // Демонстрация синхронизации
    println!("\n4. Демонстрация синхронизации:");
    let sync_demo = SyncDemo::new(2);
    sync_demo.add_data("Item 1".to_string()).await;
    sync_demo.add_data("Item 2".to_string()).await;
    println!("Данные: {:?}", sync_demo.get_data());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_demo() {
        let demo = ThreadDemo::new();
        demo.demonstrate_threads();
        let counter = demo.counter.lock().unwrap();
        assert_eq!(*counter, 5);
    }

    #[tokio::test]
    async fn test_channel_demo() {
        let mut demo = ChannelDemo::new(1);
        demo.send("test".to_string()).await.unwrap();
        assert_eq!(demo.receive().await, Some("test".to_string()));
    }

    #[test]
    fn test_atomic_demo() {
        let demo = AtomicDemo::new();
        demo.increment();
        demo.increment();
        assert_eq!(demo.get_value(), 2);
    }

    #[tokio::test]
    async fn test_sync_demo() {
        let demo = SyncDemo::new(2);
        demo.add_data("test1".to_string()).await;
        demo.add_data("test2".to_string()).await;
        assert_eq!(demo.get_data(), vec!["test1", "test2"]);
    }
} 