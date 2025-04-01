mod ownership;
mod traits;
mod async_programming;
mod error_handling;
mod testing;
mod concurrency;
mod benchmarks;
mod data_structures;
mod algorithms;
mod networking;
mod database;

#[tokio::main]
async fn main() {
    // Демонстрация системы владения
    ownership::demonstrate_ownership();

    // Демонстрация трейтов и обобщений
    traits::demonstrate_traits();

    // Демонстрация асинхронного программирования
    async_programming::demonstrate_async().await;

    // Демонстрация обработки ошибок
    error_handling::demonstrate_error_handling();

    // Демонстрация тестирования
    testing::demonstrate_testing();

    // Демонстрация многопоточности
    concurrency::demonstrate_concurrency();

    // Демонстрация бенчмарков
    benchmarks::demonstrate_benchmarks();

    // Демонстрация структур данных
    data_structures::demonstrate_data_structures();

    // Демонстрация алгоритмов
    algorithms::demonstrate_algorithms();

    // Демонстрация сетевого программирования
    networking::demonstrate_networking().await;

    // Демонстрация работы с базой данных
    database::demonstrate_database().await;
} 