//! Модуль для демонстрации метрик и мониторинга в Rust
//! 
//! Этот модуль показывает различные аспекты метрик и мониторинга:
//! - Сбор метрик
//! - Мониторинг производительности
//! - Логирование
//! - Трейсинг
//! - Алерты
//! - Дашборды
//! - Анализ производительности
//! - Отслеживание ресурсов
//! - Мониторинг состояния
//! - Метрики бизнес-логики

use std::time::{Duration, Instant};
use std::sync::Arc;
use parking_lot::Mutex;
use metrics::{counter, gauge, histogram};
use metrics_exporter_prometheus::PrometheusBuilder;
use tracing::{info, warn, error, Level};
use tracing_subscriber::FmtSubscriber;

/// Структура для демонстрации метрик
#[derive(Debug)]
pub struct MetricsDemo {
    start_time: Instant,
    request_count: Arc<Mutex<u64>>,
    active_connections: Arc<Mutex<u32>>,
}

/// Структура для демонстрации мониторинга
#[derive(Debug)]
pub struct MonitoringDemo {
    metrics: Arc<Mutex<HashMap<String, f64>>>,
    alerts: Vec<Alert>,
}

#[derive(Debug)]
struct Alert {
    name: String,
    threshold: f64,
    current_value: f64,
}

impl MetricsDemo {
    /// Создание нового экземпляра
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            request_count: Arc::new(Mutex::new(0)),
            active_connections: Arc::new(Mutex::new(0)),
        }
    }

    /// Регистрация запроса
    pub fn register_request(&self) {
        counter!("requests_total", 1);
        let mut count = self.request_count.lock();
        *count += 1;
    }

    /// Обновление активных соединений
    pub fn update_connections(&self, count: u32) {
        gauge!("active_connections", count as f64);
        let mut connections = self.active_connections.lock();
        *connections = count;
    }

    /// Измерение времени выполнения
    pub fn measure_execution_time<F, T>(&self, name: &str, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        histogram!("execution_time", duration.as_secs_f64(), "name" => name.to_string());
        result
    }
}

impl MonitoringDemo {
    /// Создание нового экземпляра
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(Mutex::new(HashMap::new())),
            alerts: Vec::new(),
        }
    }

    /// Добавление метрики
    pub fn add_metric(&self, name: String, value: f64) {
        let mut metrics = self.metrics.lock();
        metrics.insert(name, value);
    }

    /// Добавление алерта
    pub fn add_alert(&mut self, name: String, threshold: f64) {
        self.alerts.push(Alert {
            name,
            threshold,
            current_value: 0.0,
        });
    }

    /// Проверка алертов
    pub fn check_alerts(&mut self) -> Vec<String> {
        let metrics = self.metrics.lock();
        let mut triggered = Vec::new();

        for alert in &mut self.alerts {
            if let Some(&value) = metrics.get(&alert.name) {
                alert.current_value = value;
                if value > alert.threshold {
                    triggered.push(format!(
                        "Алерт {}: значение {} превышает порог {}",
                        alert.name, value, alert.threshold
                    ));
                }
            }
        }

        triggered
    }
}

/// Демонстрация метрик и мониторинга
pub fn demonstrate_metrics() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Демонстрация метрик и мониторинга ===");

    // Инициализация логгера
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .build();
    tracing::subscriber::set_global_default(subscriber)?;

    // Демонстрация метрик
    println!("\n1. Метрики:");
    let metrics = MetricsDemo::new();
    metrics.register_request();
    metrics.update_connections(5);
    metrics.measure_execution_time("test_operation", || {
        std::thread::sleep(Duration::from_millis(100));
    });

    // Демонстрация мониторинга
    println!("\n2. Мониторинг:");
    let mut monitoring = MonitoringDemo::new();
    monitoring.add_metric("cpu_usage".to_string(), 85.5);
    monitoring.add_metric("memory_usage".to_string(), 90.0);
    monitoring.add_alert("cpu_usage".to_string(), 80.0);
    monitoring.add_alert("memory_usage".to_string(), 85.0);

    let alerts = monitoring.check_alerts();
    for alert in alerts {
        warn!("{}", alert);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics() {
        let metrics = MetricsDemo::new();
        metrics.register_request();
        metrics.update_connections(3);
        assert_eq!(*metrics.request_count.lock(), 1);
        assert_eq!(*metrics.active_connections.lock(), 3);
    }

    #[test]
    fn test_monitoring() {
        let mut monitoring = MonitoringDemo::new();
        monitoring.add_metric("test".to_string(), 100.0);
        monitoring.add_alert("test".to_string(), 90.0);
        let alerts = monitoring.check_alerts();
        assert!(!alerts.is_empty());
    }
} 