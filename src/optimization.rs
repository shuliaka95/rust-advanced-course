//! Модуль для демонстрации оптимизации кода в Rust
//! 
//! Этот модуль показывает различные аспекты оптимизации:
//! - Оптимизация памяти
//! - Оптимизация производительности
//! - Оптимизация компиляции
//! - Оптимизация для встраиваемых систем
//! - Профилирование
//! - Бенчмаркинг
//! - Оптимизация алгоритмов
//! - Оптимизация структур данных
//! - Оптимизация сетевого кода
//! - Оптимизация работы с базой данных

use std::time::Instant;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use parking_lot::Mutex;

/// Структура для демонстрации оптимизации памяти
#[derive(Debug)]
pub struct MemoryOptimization {
    data: Vec<u8>,
    cache: HashMap<String, Vec<u8>>,
}

/// Структура для демонстрации оптимизации производительности
#[derive(Debug)]
pub struct PerformanceOptimization {
    buffer: VecDeque<u8>,
    pool: Arc<Mutex<Vec<Vec<u8>>>>,
}

impl MemoryOptimization {
    /// Создание нового экземпляра
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(1024),
            cache: HashMap::new(),
        }
    }

    /// Оптимизированное добавление данных
    pub fn add_data(&mut self, data: &[u8]) {
        // Предварительное выделение памяти
        if self.data.len() + data.len() > self.data.capacity() {
            self.data.reserve(data.len());
        }
        self.data.extend_from_slice(data);
    }

    /// Оптимизированное кэширование
    pub fn cache_data(&mut self, key: String, data: &[u8]) {
        // Переиспользование памяти
        if let Some(cached) = self.cache.get_mut(&key) {
            cached.clear();
            cached.extend_from_slice(data);
        } else {
            self.cache.insert(key, data.to_vec());
        }
    }

    /// Очистка кэша
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl PerformanceOptimization {
    /// Создание нового экземпляра
    pub fn new() -> Self {
        Self {
            buffer: VecDeque::with_capacity(1024),
            pool: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Оптимизированная обработка данных
    pub fn process_data(&mut self, data: &[u8]) {
        // Использование буфера
        self.buffer.extend(data.iter().copied());
        
        // Пакетная обработка
        if self.buffer.len() >= 64 {
            let mut batch = Vec::with_capacity(64);
            for _ in 0..64 {
                if let Some(value) = self.buffer.pop_front() {
                    batch.push(value);
                }
            }
            self.process_batch(&batch);
        }
    }

    /// Обработка пакета данных
    fn process_batch(&self, batch: &[u8]) {
        // Использование пула потоков
        let mut pool = self.pool.lock();
        if let Some(mut buffer) = pool.pop() {
            buffer.clear();
            buffer.extend_from_slice(batch);
            // Обработка данных
        } else {
            pool.push(batch.to_vec());
        }
    }
}

/// Демонстрация оптимизации кода
pub fn demonstrate_optimization() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Демонстрация оптимизации кода ===");

    // Демонстрация оптимизации памяти
    println!("\n1. Оптимизация памяти:");
    let mut memory_opt = MemoryOptimization::new();
    let data = vec![1, 2, 3, 4, 5];
    memory_opt.add_data(&data);
    memory_opt.cache_data("key1".to_string(), &data);
    println!("Данные добавлены и закэшированы");

    // Демонстрация оптимизации производительности
    println!("\n2. Оптимизация производительности:");
    let mut perf_opt = PerformanceOptimization::new();
    let start = Instant::now();
    perf_opt.process_data(&data);
    let duration = start.elapsed();
    println!("Время обработки: {:?}", duration);

    // Демонстрация оптимизации алгоритмов
    println!("\n3. Оптимизация алгоритмов:");
    let mut vec = vec![5, 2, 8, 1, 9];
    vec.sort_unstable(); // Более быстрая сортировка
    println!("Отсортированный вектор: {:?}", vec);

    // Демонстрация оптимизации структур данных
    println!("\n4. Оптимизация структур данных:");
    let mut map = HashMap::with_capacity(100);
    for i in 0..100 {
        map.insert(i.to_string(), i);
    }
    println!("Оптимизированная хеш-карта создана");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_optimization() {
        let mut opt = MemoryOptimization::new();
        let data = vec![1, 2, 3];
        opt.add_data(&data);
        opt.cache_data("test".to_string(), &data);
        assert_eq!(opt.data, data);
        assert_eq!(opt.cache.get("test").unwrap(), &data);
    }

    #[test]
    fn test_performance_optimization() {
        let mut opt = PerformanceOptimization::new();
        let data = vec![1, 2, 3, 4, 5];
        opt.process_data(&data);
        assert_eq!(opt.buffer.len(), data.len());
    }
} 