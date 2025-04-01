//! Библиотека для демонстрации продвинутых концепций Rust
//! 
//! Этот проект содержит примеры различных продвинутых концепций Rust:
//! - Управление памятью (стек и куча)
//! - Система владения
//! - Трейты и наследование
//! - Асинхронное программирование
//! - Обработка ошибок
//! - Многопоточность
//! - Тестирование
//! - Бенчмаркинг
//! - Структуры данных
//! - Алгоритмы
//! - Сетевое программирование
//! - Работа с базами данных
//! - Встраиваемое программирование

pub mod memory;
pub mod ownership;
pub mod traits;
pub mod async_examples;
pub mod error;
pub mod testing;
pub mod concurrency;
pub mod benchmarks;
pub mod data_structures;
pub mod algorithms;
pub mod networking;
pub mod database;
pub mod embedded;

// Реэкспорт основных типов
pub use memory::{HeapData, StackData};
pub use traits::{Animal, Dog};
pub use error::CustomError;
pub use data_structures::{ComplexData, OptimizedData};
pub use algorithms::{SortingAlgorithms, SearchingAlgorithms};
pub use networking::{HttpServer, WebSocketClient, UdpServer};
pub use database::{Database, User, UserRepository};
pub use embedded::{BitField, AtomicCounter, TimeInterval, Device, DeviceState}; 