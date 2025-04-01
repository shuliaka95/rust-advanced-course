//! Модуль для демонстрации структур данных в Rust
//! 
//! Этот модуль показывает различные структуры данных:
//! - Связные списки
//! - Деревья
//! - Графы
//! - Хеш-таблицы
//! - Очереди и стеки

use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

/// Узел связного списка
#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    /// Создание нового узла
    pub fn new(value: T) -> Self {
        Self {
            value,
            next: None,
        }
    }
}

/// Реализация связного списка
#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    /// Создание нового связного списка
    pub fn new() -> Self {
        Self { head: None }
    }

    /// Добавление элемента в начало списка
    pub fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    /// Удаление элемента из начала списка
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    /// Получение длины списка
    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;
        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }
        count
    }
}

/// Реализация бинарного дерева
#[derive(Debug)]
pub struct TreeNode<T> {
    pub value: T,
    pub left: Option<Box<TreeNode<T>>>,
    pub right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    /// Создание нового узла дерева
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    /// Добавление левого потомка
    pub fn add_left(&mut self, value: T) {
        self.left = Some(Box::new(TreeNode::new(value)));
    }

    /// Добавление правого потомка
    pub fn add_right(&mut self, value: T) {
        self.right = Some(Box::new(TreeNode::new(value)));
    }
}

/// Реализация графа
#[derive(Debug)]
pub struct Graph<T: Hash + Eq> {
    vertices: HashSet<T>,
    edges: HashMap<T, HashSet<T>>,
}

impl<T: Hash + Eq> Graph<T> {
    /// Создание нового графа
    pub fn new() -> Self {
        Self {
            vertices: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    /// Добавление вершины
    pub fn add_vertex(&mut self, vertex: T) {
        self.vertices.insert(vertex);
        self.edges.entry(vertex).or_insert_with(HashSet::new);
    }

    /// Добавление ребра
    pub fn add_edge(&mut self, from: T, to: T) {
        self.add_vertex(from.clone());
        self.add_vertex(to.clone());
        self.edges.get_mut(&from).unwrap().insert(to);
    }

    /// Получение соседей вершины
    pub fn get_neighbors(&self, vertex: &T) -> Option<&HashSet<T>> {
        self.edges.get(vertex)
    }
}

/// Реализация стека
#[derive(Debug)]
pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    /// Создание нового стека
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    /// Добавление элемента в стек
    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    /// Удаление элемента из стека
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    /// Просмотр верхнего элемента стека
    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    }
}

/// Реализация очереди
#[derive(Debug)]
pub struct Queue<T> {
    data: VecDeque<T>,
}

impl<T> Queue<T> {
    /// Создание новой очереди
    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }

    /// Добавление элемента в очередь
    pub fn enqueue(&mut self, value: T) {
        self.data.push_back(value);
    }

    /// Удаление элемента из очереди
    pub fn dequeue(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    /// Просмотр первого элемента очереди
    pub fn peek(&self) -> Option<&T> {
        self.data.front()
    }
}

/// Демонстрация структур данных
pub fn demonstrate_data_structures() -> Result<(), Box<dyn std::error::Error>> {
    // Демонстрация связного списка
    let mut list = LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    println!("Связный список: {:?}", list);

    // Демонстрация бинарного дерева
    let mut tree = TreeNode::new(1);
    tree.add_left(2);
    tree.add_right(3);
    println!("Бинарное дерево: {:?}", tree);

    // Демонстрация графа
    let mut graph = Graph::new();
    graph.add_edge(1, 2);
    graph.add_edge(2, 3);
    graph.add_edge(1, 3);
    println!("Граф: {:?}", graph);

    // Демонстрация стека
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("Стек: {:?}", stack);

    // Демонстрация очереди
    let mut queue = Queue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
    println!("Очередь: {:?}", queue);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_stack() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.peek(), Some(&2));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_queue() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        assert_eq!(queue.peek(), Some(&1));
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), None);
    }
} 