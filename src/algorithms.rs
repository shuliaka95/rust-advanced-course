//! Модуль для демонстрации алгоритмов в Rust
//! 
//! Этот модуль показывает различные алгоритмы:
//! - Сортировка
//! - Поиск
//! - Графовые алгоритмы
//! - Динамическое программирование

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::iter::FromIterator;

/// Структура для сортируемых элементов
#[derive(Debug, Clone, PartialEq)]
pub struct SortableItem {
    pub id: i32,
    pub value: f64,
    pub metadata: String,
}

impl SortableItem {
    /// Создание нового элемента
    pub fn new(id: i32, value: f64, metadata: String) -> Self {
        Self {
            id,
            value,
            metadata,
        }
    }
}

/// Реализация алгоритмов сортировки
pub struct SortingAlgorithms;

impl SortingAlgorithms {
    /// Быстрая сортировка
    pub fn quick_sort<T: Ord>(arr: &mut [T]) {
        if arr.len() <= 1 {
            return;
        }

        let pivot = partition(arr);
        let (left, right) = arr.split_at_mut(pivot);
        
        Self::quick_sort(left);
        Self::quick_sort(&mut right[1..]);
    }

    /// Сортировка слиянием
    pub fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
        if arr.len() <= 1 {
            return;
        }

        let mid = arr.len() / 2;
        let (left, right) = arr.split_at_mut(mid);
        
        Self::merge_sort(left);
        Self::merge_sort(right);
        
        merge(arr, left, right);
    }

    /// Сортировка вставками
    pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
        for i in 1..arr.len() {
            let mut j = i;
            while j > 0 && arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
                j -= 1;
            }
        }
    }

    /// Сортировка кучей (Heap Sort)
    pub fn heap_sort<T: Ord>(arr: &mut [T]) {
        let mut heap = BinaryHeap::from_iter(arr.iter());
        for i in (0..arr.len()).rev() {
            if let Some(max) = heap.pop() {
                arr[i] = *max;
            }
        }
    }
}

/// Реализация алгоритмов поиска
pub struct SearchingAlgorithms;

impl SearchingAlgorithms {
    /// Бинарный поиск
    pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
        let mut left = 0;
        let mut right = arr.len();

        while left < right {
            let mid = (left + right) / 2;
            match arr[mid].cmp(target) {
                Ordering::Equal => return Some(mid),
                Ordering::Less => left = mid + 1,
                Ordering::Greater => right = mid,
            }
        }

        None
    }

    /// Линейный поиск
    pub fn linear_search<T: PartialEq>(arr: &[T], target: &T) -> Option<usize> {
        arr.iter().position(|x| x == target)
    }

    /// Поиск с использованием хеш-таблицы
    pub fn hash_search<T: Eq + std::hash::Hash>(
        arr: &[T],
        target: &T,
    ) -> Option<usize> {
        use std::collections::HashMap;
        
        let mut hash_map = HashMap::new();
        for (i, item) in arr.iter().enumerate() {
            hash_map.insert(item, i);
        }
        
        hash_map.get(target).copied()
    }
}

/// Демонстрация алгоритмов
pub fn demonstrate_algorithms() -> Result<(), Box<dyn std::error::Error>> {
    // Демонстрация сортировки
    let mut items = vec![
        SortableItem::new(1, 3.14, "Пи".to_string()),
        SortableItem::new(2, 2.71, "e".to_string()),
        SortableItem::new(3, 1.41, "√2".to_string()),
    ];

    println!("До сортировки: {:?}", items);
    SortingAlgorithms::quick_sort(&mut items);
    println!("После быстрой сортировки: {:?}", items);

    // Демонстрация поиска
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let target = 7;

    if let Some(index) = SearchingAlgorithms::binary_search(&numbers, &target) {
        println!("Число {} найдено по индексу {}", target, index);
    } else {
        println!("Число {} не найдено", target);
    }

    Ok(())
}

// Вспомогательные функции

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot = len - 1;
    let mut store_index = 0;

    for i in 0..len - 1 {
        if arr[i] <= arr[pivot] {
            arr.swap(i, store_index);
            store_index += 1;
        }
    }

    arr.swap(pivot, store_index);
    store_index
}

fn merge<T: Ord + Clone>(arr: &mut [T], left: &[T], right: &[T]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        SortingAlgorithms::quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
    }

    #[test]
    fn test_merge_sort() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        SortingAlgorithms::merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
    }

    #[test]
    fn test_binary_search() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(SearchingAlgorithms::binary_search(&arr, &7), Some(6));
        assert_eq!(SearchingAlgorithms::binary_search(&arr, &11), None);
    }
} 