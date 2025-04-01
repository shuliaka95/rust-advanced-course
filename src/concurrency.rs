//! Демонстрация многопоточности в Rust
//! 
//! Этот модуль показывает основные концепции:
//! - Потоки
//! - Мьютексы
//! - Каналы
//! - Атомарные операции
//! - Параллельное программирование

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

pub fn demonstrate_concurrency() {
    println!("\n1. Демонстрация потоков:");
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Поток: {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });
    handle.join().unwrap();

    println!("\n2. Демонстрация мьютексов:");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..3 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Итоговое значение счетчика: {}", *counter.lock().unwrap());

    println!("\n3. Демонстрация каналов:");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Привет из потока!");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Получено: {}", received);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..3 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(*counter.lock().unwrap(), 3);
    }

    #[test]
    fn test_channel() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            tx.send(42).unwrap();
        });

        assert_eq!(rx.recv().unwrap(), 42);
    }
} 