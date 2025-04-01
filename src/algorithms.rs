//! Демонстрация алгоритмов в Rust
//! 
//! Этот модуль показывает основные концепции:
//! - Поиск
//! - Сортировка
//! - Обход графов
//! - Динамическое программирование
//! - Жадные алгоритмы

use std::collections::{HashMap, HashSet};

// Бинарный поиск
fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = (left + right) / 2;
        match arr[mid].cmp(target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
        }
    }
    None
}

// Сортировка пузырьком
fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

// Обход графа в глубину
fn dfs(graph: &HashMap<i32, Vec<i32>>, start: i32) -> Vec<i32> {
    let mut visited = HashSet::new();
    let mut result = Vec::new();

    fn dfs_recursive(
        graph: &HashMap<i32, Vec<i32>>,
        node: i32,
        visited: &mut HashSet<i32>,
        result: &mut Vec<i32>,
    ) {
        if visited.contains(&node) {
            return;
        }
        visited.insert(node);
        result.push(node);
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                dfs_recursive(graph, neighbor, visited, result);
            }
        }
    }

    dfs_recursive(graph, start, &mut visited, &mut result);
    result
}

// Динамическое программирование: числа Фибоначчи
fn fibonacci_dp(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    let mut dp = vec![0; (n + 1) as usize];
    dp[0] = 0;
    dp[1] = 1;
    for i in 2..=n as usize {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    dp[n as usize]
}

// Жадный алгоритм: размен монет
fn coin_change(coins: &[u32], amount: u32) -> Option<Vec<u32>> {
    let mut result = Vec::new();
    let mut remaining = amount;
    let mut coins = coins.to_vec();
    coins.sort_by(|a, b| b.cmp(a)); // Сортировка по убыванию

    for &coin in &coins {
        while remaining >= coin {
            result.push(coin);
            remaining -= coin;
        }
    }

    if remaining == 0 {
        Some(result)
    } else {
        None
    }
}

pub fn demonstrate_algorithms() {
    println!("\n1. Демонстрация бинарного поиска:");
    let arr = vec![1, 3, 5, 7, 9];
    let target = 5;
    match binary_search(&arr, &target) {
        Some(index) => println!("Число {} найдено по индексу {}", target, index),
        None => println!("Число {} не найдено", target),
    }

    println!("\n2. Демонстрация сортировки пузырьком:");
    let mut arr = vec![5, 3, 8, 4, 2];
    println!("До сортировки: {:?}", arr);
    bubble_sort(&mut arr);
    println!("После сортировки: {:?}", arr);

    println!("\n3. Демонстрация обхода графа в глубину:");
    let mut graph = HashMap::new();
    graph.insert(1, vec![2, 3]);
    graph.insert(2, vec![4, 5]);
    graph.insert(3, vec![6]);
    graph.insert(4, vec![]);
    graph.insert(5, vec![]);
    graph.insert(6, vec![]);
    let result = dfs(&graph, 1);
    println!("Обход графа: {:?}", result);

    println!("\n4. Демонстрация динамического программирования:");
    let n = 20;
    println!("Фибоначчи({}) = {}", n, fibonacci_dp(n));

    println!("\n5. Демонстрация жадного алгоритма:");
    let coins = vec![1, 5, 10, 25];
    let amount = 67;
    match coin_change(&coins, amount) {
        Some(result) => println!("Размен {}: {:?}", amount, result),
        None => println!("Невозможно разменять {}", amount),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let arr = vec![1, 3, 5, 7, 9];
        assert_eq!(binary_search(&arr, &5), Some(2));
        assert_eq!(binary_search(&arr, &4), None);
    }

    #[test]
    fn test_bubble_sort() {
        let mut arr = vec![5, 3, 8, 4, 2];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![2, 3, 4, 5, 8]);
    }

    #[test]
    fn test_fibonacci_dp() {
        assert_eq!(fibonacci_dp(0), 0);
        assert_eq!(fibonacci_dp(1), 1);
        assert_eq!(fibonacci_dp(2), 1);
        assert_eq!(fibonacci_dp(3), 2);
    }

    #[test]
    fn test_coin_change() {
        let coins = vec![1, 5, 10, 25];
        assert_eq!(coin_change(&coins, 67), Some(vec![25, 25, 10, 5, 1, 1]));
        assert_eq!(coin_change(&coins, 0), Some(vec![]));
    }
} 