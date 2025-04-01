//! Демонстрация работы с памятью в Rust
//! 
//! Этот модуль показывает основные концепции работы с памятью:
//! - Стек и куча
//! - Управление памятью
//! - Утечки памяти
//! - Оптимизация памяти
//! - Безопасность памяти

use std::alloc::{self, Layout};
use std::ptr::NonNull;

pub fn demonstrate_memory() {
    println!("\n1. Демонстрация работы со стеком:");
    let stack_data = StackData::new(42);
    println!("Данные на стеке: {:?}", stack_data);

    println!("\n2. Демонстрация работы с кучей:");
    let heap_data = HeapData::new(100);
    println!("Данные в куче: {:?}", heap_data);

    println!("\n3. Демонстрация управления памятью:");
    let mut memory_manager = MemoryManager::new();
    memory_manager.allocate(1024);
    println!("Менеджер памяти: {:?}", memory_manager);
}

#[derive(Debug)]
struct StackData {
    value: i32,
}

impl StackData {
    fn new(value: i32) -> Self {
        StackData { value }
    }
}

#[derive(Debug)]
struct HeapData {
    ptr: NonNull<u8>,
    size: usize,
}

impl HeapData {
    fn new(size: usize) -> Self {
        let layout = Layout::array::<u8>(size).unwrap();
        let ptr = unsafe { alloc::alloc(layout) };
        let ptr = NonNull::new(ptr).unwrap();
        HeapData { ptr, size }
    }
}

impl Drop for HeapData {
    fn drop(&mut self) {
        let layout = Layout::array::<u8>(self.size).unwrap();
        unsafe {
            alloc::dealloc(self.ptr.as_ptr(), layout);
        }
    }
}

#[derive(Debug)]
struct MemoryManager {
    allocations: Vec<(NonNull<u8>, Layout)>,
}

impl MemoryManager {
    fn new() -> Self {
        MemoryManager {
            allocations: Vec::new(),
        }
    }

    fn allocate(&mut self, size: usize) {
        let layout = Layout::array::<u8>(size).unwrap();
        let ptr = unsafe { alloc::alloc(layout) };
        let ptr = NonNull::new(ptr).unwrap();
        self.allocations.push((ptr, layout));
    }

    fn deallocate(&mut self, index: usize) {
        if let Some((ptr, layout)) = self.allocations.remove(index) {
            unsafe {
                alloc::dealloc(ptr.as_ptr(), layout);
            }
        }
    }
}

impl Drop for MemoryManager {
    fn drop(&mut self) {
        for (ptr, layout) in self.allocations.drain(..) {
            unsafe {
                alloc::dealloc(ptr.as_ptr(), layout);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_data() {
        let data = StackData::new(42);
        assert_eq!(data.value, 42);
    }

    #[test]
    fn test_heap_data() {
        let data = HeapData::new(100);
        assert_eq!(data.size, 100);
    }

    #[test]
    fn test_memory_manager() {
        let mut manager = MemoryManager::new();
        manager.allocate(1024);
        assert_eq!(manager.allocations.len(), 1);
        manager.deallocate(0);
        assert_eq!(manager.allocations.len(), 0);
    }
} 