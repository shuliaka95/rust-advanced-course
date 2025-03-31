//! Модуль для демонстрации бенчмарков в Rust
//! 
//! Этот модуль показывает различные аспекты бенчмарков:
//! - Модульные бенчмарки
//! - Критерии бенчмарков
//! - Асинхронные бенчмарки
//! - Измерение производительности
//! - Оптимизация кода

use std::time::Duration;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tokio::time::sleep;

/// Структура для демонстрации бенчмарков
#[derive(Debug)]
pub struct BenchmarkDemo {
    data: Vec<i32>,
}

/// Структура для демонстрации асинхронных бенчмарков
#[derive(Debug)]
pub struct AsyncBenchmarkDemo {
    data: Vec<String>,
}

impl BenchmarkDemo {
    /// Создание нового экземпляра
    pub fn new(data: Vec<i32>) -> Self {
        Self { data }
    }

    /// Линейный поиск
    pub fn linear_search(&self, target: i32) -> Option<usize> {
        self.data.iter().position(|&x| x == target)
    }

    /// Бинарный поиск
    pub fn binary_search(&self, target: i32) -> Option<usize> {
        self.data.binary_search(&target).ok()
    }

    /// Сортировка пузырьком
    pub fn bubble_sort(&mut self) {
        let n = self.data.len();
        for i in 0..n {
            for j in 0..n - i - 1 {
                if self.data[j] > self.data[j + 1] {
                    self.data.swap(j, j + 1);
                }
            }
        }
    }

    /// Быстрая сортировка
    pub fn quick_sort(&mut self) {
        self.quick_sort_helper(0, self.data.len() - 1);
    }

    fn quick_sort_helper(&mut self, low: usize, high: usize) {
        if low < high {
            let pivot = self.partition(low, high);
            self.quick_sort_helper(low, pivot - 1);
            self.quick_sort_helper(pivot + 1, high);
        }
    }

    fn partition(&mut self, low: usize, high: usize) -> usize {
        let pivot = self.data[high];
        let mut i = low;

        for j in low..high {
            if self.data[j] <= pivot {
                self.data.swap(i, j);
                i += 1;
            }
        }
        self.data.swap(i, high);
        i
    }
}

impl AsyncBenchmarkDemo {
    /// Создание нового экземпляра
    pub fn new(data: Vec<String>) -> Self {
        Self { data }
    }

    /// Асинхронная обработка данных
    pub async fn process_data(&self) -> Vec<String> {
        let mut results = Vec::new();
        for item in &self.data {
            // Имитация асинхронной операции
            sleep(Duration::from_millis(10)).await;
            results.push(item.to_uppercase());
        }
        results
    }

    /// Асинхронная фильтрация данных
    pub async fn filter_data(&self, predicate: &str) -> Vec<String> {
        let mut results = Vec::new();
        for item in &self.data {
            if item.contains(predicate) {
                // Имитация асинхронной операции
                sleep(Duration::from_millis(5)).await;
                results.push(item.clone());
            }
        }
        results
    }
}

/// Демонстрация бенчмарков
pub fn demonstrate_benchmarks() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Демонстрация бенчмарков ===");

    // Демонстрация поиска
    println!("\n1. Демонстрация поиска:");
    let demo = BenchmarkDemo::new((0..1000).collect());
    let target = 500;
    println!("Линейный поиск: {:?}", demo.linear_search(target));
    println!("Бинарный поиск: {:?}", demo.binary_search(target));

    // Демонстрация сортировки
    println!("\n2. Демонстрация сортировки:");
    let mut demo = BenchmarkDemo::new(vec![5, 2, 8, 1, 9]);
    demo.bubble_sort();
    println!("Сортировка пузырьком: {:?}", demo.data);

    let mut demo = BenchmarkDemo::new(vec![5, 2, 8, 1, 9]);
    demo.quick_sort();
    println!("Быстрая сортировка: {:?}", demo.data);

    Ok(())
}

/// Настройка бенчмарков
pub fn setup_benchmarks(c: &mut Criterion) {
    // Бенчмарк линейного поиска
    c.bench_function("linear_search", |b| {
        let demo = BenchmarkDemo::new((0..1000).collect());
        b.iter(|| demo.linear_search(black_box(500)))
    });

    // Бенчмарк бинарного поиска
    c.bench_function("binary_search", |b| {
        let demo = BenchmarkDemo::new((0..1000).collect());
        b.iter(|| demo.binary_search(black_box(500)))
    });

    // Бенчмарк сортировки пузырьком
    c.bench_function("bubble_sort", |b| {
        let mut demo = BenchmarkDemo::new((0..100).rev().collect());
        b.iter(|| demo.bubble_sort())
    });

    // Бенчмарк быстрой сортировки
    c.bench_function("quick_sort", |b| {
        let mut demo = BenchmarkDemo::new((0..100).rev().collect());
        b.iter(|| demo.quick_sort())
    });
}

/// Настройка асинхронных бенчмарков
pub fn setup_async_benchmarks(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    // Бенчмарк асинхронной обработки
    c.bench_function("async_process", |b| {
        let demo = AsyncBenchmarkDemo::new(vec!["test".to_string(); 10]);
        b.to_async(&rt).iter(|| demo.process_data())
    });

    // Бенчмарк асинхронной фильтрации
    c.bench_function("async_filter", |b| {
        let demo = AsyncBenchmarkDemo::new(vec!["test".to_string(); 10]);
        b.to_async(&rt).iter(|| demo.filter_data("test"))
    });
}

criterion_group!(benches, setup_benchmarks);
criterion_group!(async_benches, setup_async_benchmarks);
criterion_main!(benches, async_benches);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let demo = BenchmarkDemo::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(demo.linear_search(3), Some(2));
        assert_eq!(demo.binary_search(3), Some(2));
    }

    #[test]
    fn test_sort() {
        let mut demo = BenchmarkDemo::new(vec![5, 2, 8, 1, 9]);
        demo.bubble_sort();
        assert_eq!(demo.data, vec![1, 2, 5, 8, 9]);

        let mut demo = BenchmarkDemo::new(vec![5, 2, 8, 1, 9]);
        demo.quick_sort();
        assert_eq!(demo.data, vec![1, 2, 5, 8, 9]);
    }

    #[tokio::test]
    async fn test_async_operations() {
        let demo = AsyncBenchmarkDemo::new(vec!["test".to_string()]);
        let result = demo.process_data().await;
        assert_eq!(result, vec!["TEST"]);

        let result = demo.filter_data("test").await;
        assert_eq!(result, vec!["test"]);
    }
} 