//! Модуль для демонстрации работы с микроконтроллерами
//! 
//! Этот модуль показывает различные аспекты встраиваемого программирования:
//! - Работа с регистрами
//! - Прерывания
//! - DMA
//! - Оптимизация для встраиваемых систем
//! - Работа с периферией
//! - Реальное время
//! - Энергосбережение
//! - Отладка
//! - Безопасность

use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;

/// Структура для демонстрации работы с регистрами
#[derive(Debug)]
pub struct RegisterDemo {
    control_register: AtomicU32,
    status_register: AtomicU32,
}

/// Структура для демонстрации прерываний
#[derive(Debug)]
pub struct InterruptDemo {
    interrupt_mask: AtomicU32,
    interrupt_status: AtomicU32,
}

/// Структура для демонстрации DMA
#[derive(Debug)]
pub struct DMADemo {
    source_address: u32,
    destination_address: u32,
    transfer_size: u32,
}

/// Структура для демонстрации работы с периферией
#[derive(Debug)]
pub struct PeripheralDemo {
    gpio_config: u32,
    uart_config: u32,
    spi_config: u32,
}

impl RegisterDemo {
    /// Создание нового экземпляра
    pub fn new() -> Self {
        Self {
            control_register: AtomicU32::new(0),
            status_register: AtomicU32::new(0),
        }
    }

    /// Установка бита в регистре управления
    pub fn set_control_bit(&self, bit: u8) {
        let mask = 1 << bit;
        self.control_register.fetch_or(mask, Ordering::SeqCst);
    }

    /// Сброс бита в регистре управления
    pub fn clear_control_bit(&self, bit: u8) {
        let mask = !(1 << bit);
        self.control_register.fetch_and(mask, Ordering::SeqCst);
    }

    /// Чтение бита из регистра состояния
    pub fn read_status_bit(&self, bit: u8) -> bool {
        let mask = 1 << bit;
        (self.status_register.load(Ordering::SeqCst) & mask) != 0
    }
}

impl InterruptDemo {
    /// Создание нового экземпляра
    pub fn new() -> Self {
        Self {
            interrupt_mask: AtomicU32::new(0),
            interrupt_status: AtomicU32::new(0),
        }
    }

    /// Включение прерывания
    pub fn enable_interrupt(&self, interrupt: u8) {
        let mask = 1 << interrupt;
        self.interrupt_mask.fetch_or(mask, Ordering::SeqCst);
    }

    /// Отключение прерывания
    pub fn disable_interrupt(&self, interrupt: u8) {
        let mask = !(1 << interrupt);
        self.interrupt_mask.fetch_and(mask, Ordering::SeqCst);
    }

    /// Проверка статуса прерывания
    pub fn check_interrupt(&self, interrupt: u8) -> bool {
        let mask = 1 << interrupt;
        (self.interrupt_status.load(Ordering::SeqCst) & mask) != 0
    }

    /// Обработчик прерывания
    pub fn handle_interrupt(&self, interrupt: u8) {
        println!("Обработка прерывания {}", interrupt);
        // Здесь будет код обработки прерывания
    }
}

impl DMADemo {
    /// Создание нового экземпляра
    pub fn new(source: u32, destination: u32, size: u32) -> Self {
        Self {
            source_address: source,
            destination_address: destination,
            transfer_size: size,
        }
    }

    /// Настройка DMA
    pub fn configure(&self) {
        println!(
            "Настройка DMA: источник={}, назначение={}, размер={}",
            self.source_address, self.destination_address, self.transfer_size
        );
    }

    /// Запуск передачи
    pub fn start_transfer(&self) {
        println!("Запуск DMA передачи");
        // Здесь будет код запуска DMA
    }

    /// Проверка статуса передачи
    pub fn check_status(&self) -> bool {
        // Здесь будет код проверки статуса
        true
    }
}

impl PeripheralDemo {
    /// Создание нового экземпляра
    pub fn new() -> Self {
        Self {
            gpio_config: 0,
            uart_config: 0,
            spi_config: 0,
        }
    }

    /// Настройка GPIO
    pub fn configure_gpio(&mut self, pin: u8, mode: u8) {
        let mask = 0b11 << (pin * 2);
        let value = mode << (pin * 2);
        self.gpio_config = (self.gpio_config & !mask) | value;
        println!("Настройка GPIO: пин={}, режим={}", pin, mode);
    }

    /// Настройка UART
    pub fn configure_uart(&mut self, baud_rate: u32, data_bits: u8, stop_bits: u8) {
        self.uart_config = (baud_rate as u32) | ((data_bits as u32) << 16) | ((stop_bits as u32) << 24);
        println!(
            "Настройка UART: скорость={}, биты данных={}, стоп-биты={}",
            baud_rate, data_bits, stop_bits
        );
    }

    /// Настройка SPI
    pub fn configure_spi(&mut self, mode: u8, clock_divider: u8) {
        self.spi_config = ((mode as u32) << 8) | (clock_divider as u32);
        println!("Настройка SPI: режим={}, делитель={}", mode, clock_divider);
    }
}

/// Демонстрация работы с микроконтроллерами
pub fn demonstrate_embedded_concepts() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Демонстрация работы с микроконтроллерами ===");

    // Демонстрация работы с регистрами
    println!("\n1. Работа с регистрами:");
    let register_demo = RegisterDemo::new();
    register_demo.set_control_bit(0);
    register_demo.set_control_bit(1);
    println!("Бит 0 установлен: {}", register_demo.read_status_bit(0));
    println!("Бит 1 установлен: {}", register_demo.read_status_bit(1));

    // Демонстрация прерываний
    println!("\n2. Прерывания:");
    let interrupt_demo = InterruptDemo::new();
    interrupt_demo.enable_interrupt(0);
    interrupt_demo.enable_interrupt(1);
    println!("Прерывание 0 включено: {}", interrupt_demo.check_interrupt(0));
    println!("Прерывание 1 включено: {}", interrupt_demo.check_interrupt(1));

    // Демонстрация DMA
    println!("\n3. DMA:");
    let dma_demo = DMADemo::new(0x1000, 0x2000, 1024);
    dma_demo.configure();
    dma_demo.start_transfer();
    println!("Статус передачи: {}", dma_demo.check_status());

    // Демонстрация работы с периферией
    println!("\n4. Периферия:");
    let mut peripheral_demo = PeripheralDemo::new();
    peripheral_demo.configure_gpio(0, 1); // Настройка GPIO
    peripheral_demo.configure_uart(9600, 8, 1); // Настройка UART
    peripheral_demo.configure_spi(0, 4); // Настройка SPI

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_operations() {
        let demo = RegisterDemo::new();
        demo.set_control_bit(0);
        assert!(demo.read_status_bit(0));
        demo.clear_control_bit(0);
        assert!(!demo.read_status_bit(0));
    }

    #[test]
    fn test_interrupt_operations() {
        let demo = InterruptDemo::new();
        demo.enable_interrupt(0);
        assert!(demo.check_interrupt(0));
        demo.disable_interrupt(0);
        assert!(!demo.check_interrupt(0));
    }

    #[test]
    fn test_dma_operations() {
        let demo = DMADemo::new(0x1000, 0x2000, 1024);
        assert_eq!(demo.source_address, 0x1000);
        assert_eq!(demo.destination_address, 0x2000);
        assert_eq!(demo.transfer_size, 1024);
    }

    #[test]
    fn test_peripheral_operations() {
        let mut demo = PeripheralDemo::new();
        demo.configure_gpio(0, 1);
        assert_eq!(demo.gpio_config & 0b11, 1);
    }
} 