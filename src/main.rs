//! # Продвинутый курс по Rust
//! 
//! Этот проект демонстрирует основные и продвинутые концепции Rust.
//! Каждый модуль содержит подробные примеры и объяснения.

mod memory;
mod ownership;
mod traits;
mod async_examples;
mod error;
mod testing;
mod concurrency;
mod benchmarks;
mod data_structures;
mod algorithms;
mod networking;
mod database;
mod embedded;
mod optimization;
mod security;
mod metrics;

use memory::demonstrate_memory_differences;
use ownership::demonstrate_ownership;
use traits::demonstrate_traits;
use async_examples::demonstrate_async;
use error::demonstrate_error_handling;
use testing::demonstrate_testing;
use concurrency::demonstrate_concurrency;
use benchmarks::demonstrate_benchmarks;
use data_structures::demonstrate_data_structures;
use algorithms::demonstrate_algorithms;
use networking::{demonstrate_http_server, demonstrate_websocket_client, demonstrate_udp_server};
use database::{demonstrate_crud_operations, demonstrate_transactions};
use embedded::demonstrate_embedded_concepts;
use optimization::demonstrate_optimization;
use security::demonstrate_security;
use metrics::demonstrate_metrics;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Инициализация логгера
    env_logger::init();
    
    println!("Демонстрация продвинутых концепций Rust\n");

    // Демонстрация управления памятью
    println!("\n=== Демонстрация управления памятью ===");
    demonstrate_memory_differences()?;

    // Демонстрация системы владения
    println!("\n=== Демонстрация системы владения ===");
    demonstrate_ownership()?;

    // Демонстрация трейтов
    println!("\n=== Демонстрация трейтов ===");
    demonstrate_traits()?;

    // Демонстрация асинхронного программирования
    println!("\n=== Демонстрация асинхронного программирования ===");
    demonstrate_async().await?;

    // Демонстрация обработки ошибок
    println!("\n=== Демонстрация обработки ошибок ===");
    demonstrate_error_handling()?;

    // Демонстрация тестирования
    println!("\n=== Демонстрация тестирования ===");
    demonstrate_testing()?;

    // Демонстрация многопоточности
    println!("\n=== Демонстрация многопоточности ===");
    demonstrate_concurrency()?;

    // Демонстрация бенчмарков
    println!("\n=== Демонстрация бенчмарков ===");
    demonstrate_benchmarks()?;

    // Демонстрация структур данных
    println!("\n=== Демонстрация структур данных ===");
    demonstrate_data_structures()?;

    // Демонстрация алгоритмов
    println!("\n=== Демонстрация алгоритмов ===");
    demonstrate_algorithms()?;

    // Демонстрация сетевого программирования
    println!("\n=== Демонстрация сетевого программирования ===");
    demonstrate_http_server().await?;
    demonstrate_websocket_client().await?;
    demonstrate_udp_server().await?;

    // Демонстрация работы с базой данных
    println!("\n=== Демонстрация работы с базой данных ===");
    demonstrate_crud_operations().await?;
    demonstrate_transactions().await?;

    // Демонстрация встраиваемого программирования
    println!("\n=== Демонстрация встраиваемого программирования ===");
    demonstrate_embedded_concepts()?;

    // Демонстрация оптимизации
    println!("\n=== Демонстрация оптимизации ===");
    demonstrate_optimization()?;

    // Демонстрация безопасности
    println!("\n=== Демонстрация безопасности ===");
    demonstrate_security()?;

    // Демонстрация метрик и мониторинга
    println!("\n=== Демонстрация метрик и мониторинга ===");
    demonstrate_metrics()?;

    Ok(())
}
