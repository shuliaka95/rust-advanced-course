//! Демонстрация встраиваемого программирования в Rust
//! 
//! Этот модуль показывает основные концепции встраиваемого программирования:
//! - Регистры
//! - Прерывания
//! - DMA
//! - Периферийные устройства
//! - Оптимизация памяти

use std::sync::atomic::{AtomicU32, Ordering};

pub fn demonstrate_embedded() {
    println!("\n1. Демонстрация работы с регистрами:");
    let mut register = Register::new();
    register.set_bit(0, true);
    register.set_bit(1, true);
    println!("Регистр: {:?}", register);

    println!("\n2. Демонстрация прерываний:");
    let mut interrupt = Interrupt::new();
    interrupt.enable();
    println!("Прерывание включено: {}", interrupt.is_enabled());

    println!("\n3. Демонстрация DMA:");
    let mut dma = DMA::new();
    dma.start_transfer(0x1000, 0x2000, 1024);
    println!("DMA: {:?}", dma);

    println!("\n4. Демонстрация периферийных устройств:");
    let mut device = Device::new();
    device.initialize();
    println!("Устройство: {:?}", device);
}

#[derive(Debug)]
struct Register {
    value: u32,
}

impl Register {
    fn new() -> Self {
        Register { value: 0 }
    }

    fn set_bit(&mut self, bit: u8, value: bool) {
        if value {
            self.value |= 1 << bit;
        } else {
            self.value &= !(1 << bit);
        }
    }

    fn get_bit(&self, bit: u8) -> bool {
        (self.value & (1 << bit)) != 0
    }
}

#[derive(Debug)]
struct Interrupt {
    enabled: AtomicU32,
}

impl Interrupt {
    fn new() -> Self {
        Interrupt {
            enabled: AtomicU32::new(0),
        }
    }

    fn enable(&mut self) {
        self.enabled.store(1, Ordering::SeqCst);
    }

    fn disable(&mut self) {
        self.enabled.store(0, Ordering::SeqCst);
    }

    fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::SeqCst) != 0
    }
}

#[derive(Debug)]
struct DMA {
    source: u32,
    destination: u32,
    length: u32,
    active: bool,
}

impl DMA {
    fn new() -> Self {
        DMA {
            source: 0,
            destination: 0,
            length: 0,
            active: false,
        }
    }

    fn start_transfer(&mut self, source: u32, destination: u32, length: u32) {
        self.source = source;
        self.destination = destination;
        self.length = length;
        self.active = true;
    }

    fn stop_transfer(&mut self) {
        self.active = false;
    }
}

#[derive(Debug)]
struct Device {
    initialized: bool,
    status: u32,
}

impl Device {
    fn new() -> Self {
        Device {
            initialized: false,
            status: 0,
        }
    }

    fn initialize(&mut self) {
        self.initialized = true;
        self.status = 1;
    }

    fn read_status(&self) -> u32 {
        self.status
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut reg = Register::new();
        reg.set_bit(0, true);
        assert!(reg.get_bit(0));
        reg.set_bit(0, false);
        assert!(!reg.get_bit(0));
    }

    #[test]
    fn test_interrupt() {
        let mut int = Interrupt::new();
        assert!(!int.is_enabled());
        int.enable();
        assert!(int.is_enabled());
        int.disable();
        assert!(!int.is_enabled());
    }

    #[test]
    fn test_dma() {
        let mut dma = DMA::new();
        dma.start_transfer(0x1000, 0x2000, 1024);
        assert!(dma.active);
        assert_eq!(dma.source, 0x1000);
        assert_eq!(dma.destination, 0x2000);
        assert_eq!(dma.length, 1024);
    }

    #[test]
    fn test_device() {
        let mut device = Device::new();
        assert!(!device.initialized);
        device.initialize();
        assert!(device.initialized);
        assert_eq!(device.read_status(), 1);
    }
} 