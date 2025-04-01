//! Демонстрация трейтов и обобщений в Rust
//! 
//! Этот модуль показывает основные концепции:
//! - Трейты
//! - Обобщенные типы
//! - Трейт-объекты
//! - Ассоциированные типы
//! - Расширенные возможности трейтов

// Определение трейта
trait Animal {
    fn make_sound(&self) -> String;
    fn name(&self) -> &str;
}

// Реализация трейта для структуры
struct Dog {
    name: String,
}

impl Animal for Dog {
    fn make_sound(&self) -> String {
        "Гав!".to_string()
    }

    fn name(&self) -> &str {
        &self.name
    }
}

struct Cat {
    name: String,
}

impl Animal for Cat {
    fn make_sound(&self) -> String {
        "Мяу!".to_string()
    }

    fn name(&self) -> &str {
        &self.name
    }
}

// Обобщенная функция
fn print_animal_info<T: Animal>(animal: &T) {
    println!("{} говорит: {}", animal.name(), animal.make_sound());
}

// Трейт с ассоциированным типом
trait Container {
    type Item;
    fn add(&mut self, item: Self::Item);
    fn get(&self, index: usize) -> Option<&Self::Item>;
}

struct VecContainer<T> {
    items: Vec<T>,
}

impl<T> Container for VecContainer<T> {
    type Item = T;

    fn add(&mut self, item: T) {
        self.items.push(item);
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }
}

pub fn demonstrate_traits() {
    println!("\n1. Демонстрация трейтов:");
    let dog = Dog {
        name: "Рекс".to_string(),
    };
    let cat = Cat {
        name: "Мурка".to_string(),
    };
    print_animal_info(&dog);
    print_animal_info(&cat);

    println!("\n2. Демонстрация обобщенных типов:");
    let mut container = VecContainer {
        items: Vec::new(),
    };
    container.add(1);
    container.add(2);
    container.add(3);
    println!("Элемент с индексом 1: {:?}", container.get(1));

    println!("\n3. Демонстрация трейт-объектов:");
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(dog),
        Box::new(cat),
    ];
    for animal in animals {
        println!("{} говорит: {}", animal.name(), animal.make_sound());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dog() {
        let dog = Dog {
            name: "Рекс".to_string(),
        };
        assert_eq!(dog.make_sound(), "Гав!");
        assert_eq!(dog.name(), "Рекс");
    }

    #[test]
    fn test_cat() {
        let cat = Cat {
            name: "Мурка".to_string(),
        };
        assert_eq!(cat.make_sound(), "Мяу!");
        assert_eq!(cat.name(), "Мурка");
    }

    #[test]
    fn test_container() {
        let mut container = VecContainer {
            items: Vec::new(),
        };
        container.add(1);
        container.add(2);
        assert_eq!(container.get(0), Some(&1));
        assert_eq!(container.get(1), Some(&2));
        assert_eq!(container.get(2), None);
    }
} 