//! Демонстрация бенчмарков в Rust
//! 
//! Этот модуль показывает основные концепции:
//! - Измерение производительности
//! - Оптимизация кода
//! - Профилирование
//! - Метрики
//! - Сравнение алгоритмов

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn demonstrate_benchmarks() {
    println!("\n1. Демонстрация вычисления чисел Фибоначчи:");
    let n = 20;
    println!("Фибоначчи({}) = {}", n, fibonacci(n));

    println!("\n2. Демонстрация суммирования чисел:");
    let numbers: Vec<i32> = (1..=1000).collect();
    println!("Сумма чисел от 1 до 1000:");
    println!("Итеративный метод: {}", sum_iterative(&numbers));
    println!("Рекурсивный метод: {}", sum_recursive(&numbers));
}

// Функция для вычисления чисел Фибоначчи
fn fibonacci(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    let mut prev = 0;
    let mut curr = 1;
    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    curr
}

// Итеративное суммирование
fn sum_iterative(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

// Рекурсивное суммирование
fn sum_recursive(numbers: &[i32]) -> i32 {
    if numbers.is_empty() {
        0
    } else {
        numbers[0] + sum_recursive(&numbers[1..])
    }
}

// Бенчмарки
fn fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("fibonacci 20", |b| {
        b.iter(|| fibonacci(black_box(20)))
    });
}

fn sum_benchmark(c: &mut Criterion) {
    let numbers: Vec<i32> = (1..=1000).collect();
    c.bench_function("sum_iterative", |b| {
        b.iter(|| sum_iterative(black_box(&numbers)))
    });
    c.bench_function("sum_recursive", |b| {
        b.iter(|| sum_recursive(black_box(&numbers)))
    });
}

criterion_group!(benches, fibonacci_benchmark, sum_benchmark);
criterion_main!(benches);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
    }

    #[test]
    fn test_sum() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(sum_iterative(&numbers), 15);
        assert_eq!(sum_recursive(&numbers), 15);
    }
} 