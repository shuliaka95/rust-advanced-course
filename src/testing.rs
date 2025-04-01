//! Модуль для демонстрации тестирования в Rust
//! 
//! Этот модуль показывает различные аспекты тестирования:
//! - Модульные тесты
//! - Интеграционные тесты
//! - Асинхронные тесты
//! - Тесты с моками
//! - Тесты производительности

use std::time::Duration;
use tokio::time::sleep;
use mockall::predicate::*;
use mockall::automock;

/// Трейт для демонстрации моков
#[automock]
pub trait DataProvider {
    fn get_data(&self) -> Vec<String>;
    fn process_data(&self, data: &str) -> Result<String, String>;
}

/// Структура для демонстрации тестирования
#[derive(Debug)]
pub struct TestDemo {
    data: Vec<String>,
}

/// Структура для демонстрации асинхронного тестирования
#[derive(Debug)]
pub struct AsyncTestDemo {
    provider: Box<dyn DataProvider>,
}

impl TestDemo {
    /// Создание нового экземпляра
    pub fn new(data: Vec<String>) -> Self {
        Self { data }
    }

    /// Фильтрация данных
    pub fn filter_data(&self, predicate: &str) -> Vec<String> {
        self.data
            .iter()
            .filter(|item| item.contains(predicate))
            .cloned()
            .collect()
    }

    /// Сортировка данных
    pub fn sort_data(&self) -> Vec<String> {
        let mut sorted = self.data.clone();
        sorted.sort();
        sorted
    }

    /// Объединение данных
    pub fn combine_data(&self, other: &[String]) -> Vec<String> {
        let mut combined = self.data.clone();
        combined.extend_from_slice(other);
        combined
    }
}

impl AsyncTestDemo {
    /// Создание нового экземпляра
    pub fn new(provider: Box<dyn DataProvider>) -> Self {
        Self { provider }
    }

    /// Асинхронная обработка данных
    pub async fn process_data(&self) -> Result<Vec<String>, String> {
        let data = self.provider.get_data();
        let mut results = Vec::new();

        for item in data {
            // Имитация асинхронной операции
            sleep(Duration::from_millis(100)).await;
            
            match self.provider.process_data(&item) {
                Ok(result) => results.push(result),
                Err(e) => return Err(e),
            }
        }

        Ok(results)
    }

    /// Асинхронная фильтрация данных
    pub async fn filter_data(&self, predicate: &str) -> Result<Vec<String>, String> {
        let data = self.provider.get_data();
        let mut results = Vec::new();

        for item in data {
            if item.contains(predicate) {
                // Имитация асинхронной операции
                sleep(Duration::from_millis(50)).await;
                
                match self.provider.process_data(&item) {
                    Ok(result) => results.push(result),
                    Err(e) => return Err(e),
                }
            }
        }

        Ok(results)
    }
}

/// Демонстрация тестирования
pub fn demonstrate_testing() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Демонстрация тестирования ===");

    // Демонстрация базового тестирования
    println!("\n1. Базовое тестирование:");
    let demo = TestDemo::new(vec![
        "test1".to_string(),
        "test2".to_string(),
        "test3".to_string(),
    ]);
    println!("Отфильтрованные данные: {:?}", demo.filter_data("test"));
    println!("Отсортированные данные: {:?}", demo.sort_data());
    println!(
        "Объединенные данные: {:?}",
        demo.combine_data(&["test4".to_string(), "test5".to_string()])
    );

    Ok(())
}

/// Демонстрация асинхронного тестирования
pub async fn demonstrate_async_testing() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Демонстрация асинхронного тестирования ===");

    // Создание мока провайдера данных
    let mut mock_provider = MockDataProvider::new();
    mock_provider
        .expect_get_data()
        .returning(|| vec!["test1".to_string(), "test2".to_string()]);
    mock_provider
        .expect_process_data()
        .returning(|data| Ok(format!("processed_{}", data)));

    let demo = AsyncTestDemo::new(Box::new(mock_provider));

    // Демонстрация асинхронной обработки
    println!("\n1. Асинхронная обработка:");
    match demo.process_data().await {
        Ok(results) => println!("Результаты обработки: {:?}", results),
        Err(e) => println!("Ошибка обработки: {}", e),
    }

    // Демонстрация асинхронной фильтрации
    println!("\n2. Асинхронная фильтрация:");
    match demo.filter_data("test").await {
        Ok(results) => println!("Отфильтрованные результаты: {:?}", results),
        Err(e) => println!("Ошибка фильтрации: {}", e),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    #[test]
    fn test_filter_data() {
        let demo = TestDemo::new(vec![
            "test1".to_string(),
            "test2".to_string(),
            "other".to_string(),
        ]);
        let filtered = demo.filter_data("test");
        assert_eq!(filtered.len(), 2);
        assert!(filtered.contains(&"test1".to_string()));
        assert!(filtered.contains(&"test2".to_string()));
    }

    #[test]
    fn test_sort_data() {
        let demo = TestDemo::new(vec![
            "c".to_string(),
            "a".to_string(),
            "b".to_string(),
        ]);
        let sorted = demo.sort_data();
        assert_eq!(sorted, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_combine_data() {
        let demo = TestDemo::new(vec!["a".to_string(), "b".to_string()]);
        let combined = demo.combine_data(&["c".to_string(), "d".to_string()]);
        assert_eq!(combined, vec!["a", "b", "c", "d"]);
    }

    #[tokio::test]
    async fn test_async_process_data() {
        let mut mock_provider = MockDataProvider::new();
        mock_provider
            .expect_get_data()
            .returning(|| vec!["test".to_string()]);
        mock_provider
            .expect_process_data()
            .returning(|data| Ok(format!("processed_{}", data)));

        let demo = AsyncTestDemo::new(Box::new(mock_provider));
        let result = demo.process_data().await.unwrap();
        assert_eq!(result, vec!["processed_test"]);
    }

    #[tokio::test]
    async fn test_async_filter_data() {
        let mut mock_provider = MockDataProvider::new();
        mock_provider
            .expect_get_data()
            .returning(|| vec!["test1".to_string(), "other".to_string()]);
        mock_provider
            .expect_process_data()
            .returning(|data| Ok(format!("processed_{}", data)));

        let demo = AsyncTestDemo::new(Box::new(mock_provider));
        let result = demo.filter_data("test").await.unwrap();
        assert_eq!(result, vec!["processed_test1"]);
    }
} 