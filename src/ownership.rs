//! Демонстрация системы владения в Rust
//! 
//! Этот модуль показывает основные концепции:
//! - Владение (Ownership)
//! - Заимствование (Borrowing)
//! - Времена жизни (Lifetimes)
//! - Ссылки
//! - Срезы

pub fn demonstrate_ownership() {
    println!("\n1. Демонстрация владения:");
    let s1 = String::from("hello");
    let s2 = s1; // s1 перемещается в s2
    // println!("{}", s1); // Ошибка! s1 больше не действителен
    println!("{}", s2);

    println!("\n2. Демонстрация клонирования:");
    let s3 = String::from("hello");
    let s4 = s3.clone(); // s3 клонируется в s4
    println!("s3 = {}, s4 = {}", s3, s4);

    println!("\n3. Демонстрация заимствования:");
    let s5 = String::from("hello");
    let len = calculate_length(&s5); // s5 заимствуется
    println!("Длина '{}' равна {}.", s5, len);

    println!("\n4. Демонстрация изменяемого заимствования:");
    let mut s6 = String::from("hello");
    change(&mut s6); // s6 заимствуется как изменяемая ссылка
    println!("{}", s6);

    println!("\n5. Демонстрация срезов:");
    let s7 = String::from("hello world");
    let hello = &s7[0..5];
    let world = &s7[6..11];
    println!("{} {}", hello, world);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_length() {
        let s = String::from("hello");
        assert_eq!(calculate_length(&s), 5);
    }

    #[test]
    fn test_change() {
        let mut s = String::from("hello");
        change(&mut s);
        assert_eq!(s, "hello, world");
    }
} 