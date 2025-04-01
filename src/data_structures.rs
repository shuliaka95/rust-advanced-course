//! Демонстрация структур данных в Rust
//! 
//! Этот модуль показывает основные концепции:
//! - Стек
//! - Очередь
//! - Связанный список
//! - Дерево
//! - Граф
//! - Хеш-таблица

use std::collections::{HashMap, VecDeque};

// Реализация стека
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            items: Vec::new(),
        }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

// Реализация очереди
struct Queue<T> {
    items: VecDeque<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue {
            items: VecDeque::new(),
        }
    }

    fn enqueue(&mut self, item: T) {
        self.items.push_back(item);
    }

    fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    fn peek(&self) -> Option<&T> {
        self.items.front()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

// Реализация связанного списка
#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

pub fn demonstrate_data_structures() {
    println!("\n1. Демонстрация стека:");
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("Верхний элемент стека: {:?}", stack.peek());
    while let Some(item) = stack.pop() {
        println!("Извлечено из стека: {}", item);
    }

    println!("\n2. Демонстрация очереди:");
    let mut queue = Queue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
    println!("Первый элемент очереди: {:?}", queue.peek());
    while let Some(item) = queue.dequeue() {
        println!("Извлечено из очереди: {}", item);
    }

    println!("\n3. Демонстрация связанного списка:");
    let mut list = LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    while let Some(item) = list.pop_front() {
        println!("Извлечено из списка: {}", item);
    }

    println!("\n4. Демонстрация хеш-таблицы:");
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    map.insert("key3", "value3");
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_queue() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_linked_list() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }
} 