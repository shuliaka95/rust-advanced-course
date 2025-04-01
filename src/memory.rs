//! Модуль для демонстрации управления памятью в Rust
//! 
//! Этот модуль показывает различные аспекты управления памятью:
//! - Различия между стеком и кучей
//! - Умные указатели (Box, Rc, Arc)
//! - Оптимизация использования памяти
//! - Паттерны управления памятью
//! - Работа с небезопасным кодом

use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use std::mem;
use std::ptr;

/// Структура для демонстрации размещения данных в стеке
#[derive(Debug)]
pub struct StackData {
    small_array: [u8; 100],
    number: i32,
    flag: bool,
}

/// Структура для демонстрации размещения данных в куче
#[derive(Debug)]
pub struct HeapData {
    large_array: Vec<u8>,
    string: String,
    boxed_number: Box<i32>,
}

/// Структура для демонстрации умных указателей
#[derive(Debug)]
pub struct SmartPointerDemo {
    boxed_data: Box<[u8]>,
    rc_data: Rc<String>,
    arc_data: Arc<Vec<i32>>,
    refcell_data: RefCell<Vec<u8>>,
}

/// Структура для демонстрации небезопасного кода
#[derive(Debug)]
pub struct UnsafeDemo {
    raw_ptr: *mut u8,
    size: usize,
}

impl StackData {
    /// Создание данных в стеке
    pub fn new() -> Self {
        Self {
            small_array: [0; 100],
            number: 42,
            flag: true,
        }
    }

    /// Демонстрация копирования данных в стеке
    pub fn demonstrate_stack_copy(&self) {
        let copy = *self;
        println!("Данные скопированы в стеке: {:?}", copy);
    }
}

impl HeapData {
    /// Создание данных в куче
    pub fn new() -> Self {
        Self {
            large_array: vec![0; 1000],
            string: String::from("Hello, Heap!"),
            boxed_number: Box::new(42),
        }
    }

    /// Демонстрация перемещения данных из кучи
    pub fn demonstrate_heap_move(self) {
        println!("Данные перемещены из кучи: {:?}", self);
    }
}

impl SmartPointerDemo {
    /// Создание демонстрации умных указателей
    pub fn new() -> Self {
        Self {
            boxed_data: Box::new([1, 2, 3, 4, 5]),
            rc_data: Rc::new(String::from("Shared String")),
            arc_data: Arc::new(vec![1, 2, 3, 4, 5]),
            refcell_data: RefCell::new(vec![1, 2, 3]),
        }
    }

    /// Демонстрация использования Box
    pub fn demonstrate_box(&self) {
        let boxed_copy = self.boxed_data.clone();
        println!("Boxed data: {:?}", boxed_copy);
    }

    /// Демонстрация использования Rc
    pub fn demonstrate_rc(&self) {
        let rc_clone = Rc::clone(&this.rc_data);
        println!("Rc data: {:?}", rc_clone);
    }

    /// Демонстрация использования Arc
    pub fn demonstrate_arc(&self) {
        let arc_clone = Arc::clone(&this.arc_data);
        println!("Arc data: {:?}", arc_clone);
    }

    /// Демонстрация использования RefCell
    pub fn demonstrate_refcell(&self) {
        let mut borrowed = this.refcell_data.borrow_mut();
        borrowed.push(4);
        println!("RefCell data: {:?}", borrowed);
    }
}

impl UnsafeDemo {
    /// Создание демонстрации небезопасного кода
    pub fn new(size: usize) -> Self {
        let raw_ptr = unsafe {
            let ptr = ptr::alloc(mem::Layout::array::<u8>(size).unwrap());
            ptr::write_bytes(ptr, 0, size);
            ptr
        };
        Self {
            raw_ptr,
            size,
        }
    }

    /// Демонстрация небезопасных операций
    pub unsafe fn demonstrate_unsafe(&self) {
        println!("Работа с небезопасным указателем: {:?}", self.raw_ptr);
        // Здесь можно выполнить небезопасные операции
    }
}

impl Drop for UnsafeDemo {
    /// Освобождение памяти при удалении
    fn drop(&mut self) {
        unsafe {
            ptr::dealloc(
                self.raw_ptr,
                mem::Layout::array::<u8>(self.size).unwrap(),
            );
        }
    }
}

/// Демонстрация различий в управлении памятью
pub fn demonstrate_memory_differences() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Демонстрация управления памятью ===");

    // Демонстрация стека
    println!("\n1. Данные в стеке:");
    let stack_data = StackData::new();
    stack_data.demonstrate_stack_copy();

    // Демонстрация кучи
    println!("\n2. Данные в куче:");
    let heap_data = HeapData::new();
    heap_data.demonstrate_heap_move();

    // Демонстрация умных указателей
    println!("\n3. Умные указатели:");
    let smart_demo = SmartPointerDemo::new();
    smart_demo.demonstrate_box();
    smart_demo.demonstrate_rc();
    smart_demo.demonstrate_arc();
    smart_demo.demonstrate_refcell();

    // Демонстрация небезопасного кода
    println!("\n4. Небезопасный код:");
    let unsafe_demo = UnsafeDemo::new(100);
    unsafe {
        unsafe_demo.demonstrate_unsafe();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_data() {
        let data = StackData::new();
        assert_eq!(data.number, 42);
        assert!(data.flag);
        assert_eq!(data.small_array.len(), 100);
    }

    #[test]
    fn test_heap_data() {
        let data = HeapData::new();
        assert_eq!(*data.boxed_number, 42);
        assert_eq!(data.string, "Hello, Heap!");
        assert_eq!(data.large_array.len(), 1000);
    }

    #[test]
    fn test_smart_pointers() {
        let demo = SmartPointerDemo::new();
        assert_eq!(demo.boxed_data.len(), 5);
        assert_eq!(demo.rc_data.len(), 12);
        assert_eq!(demo.arc_data.len(), 5);
        assert_eq!(demo.refcell_data.borrow().len(), 3);
    }

    #[test]
    fn test_unsafe_demo() {
        let demo = UnsafeDemo::new(100);
        unsafe {
            assert!(!demo.raw_ptr.is_null());
        }
    }
} 