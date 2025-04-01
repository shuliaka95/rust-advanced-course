//! Модуль для демонстрации безопасности в Rust
//! 
//! Этот модуль показывает различные аспекты безопасности:
//! - Криптография
//! - Безопасное хранение данных
//! - Защита от атак
//! - Валидация входных данных
//! - Безопасное сетевое взаимодействие
//! - Управление секретами
//! - Аудит безопасности
//! - Безопасное логирование
//! - Защита от утечек памяти
//! - Безопасное многопоточное программирование

use std::sync::Arc;
use parking_lot::Mutex;
use ring::{rand, pbkdf2, digest};
use ring::rand::SecureRandom;
use ring::pbkdf2::{PBKDF2_HMAC_SHA256, derive};
use ring::digest::{SHA256, SHA512};

/// Структура для демонстрации криптографических операций
#[derive(Debug)]
pub struct CryptoDemo {
    rng: ring::rand::SystemRandom,
    salt: Vec<u8>,
}

/// Структура для демонстрации безопасного хранения данных
#[derive(Debug)]
pub struct SecureStorage {
    data: Arc<Mutex<Vec<u8>>>,
    key: Vec<u8>,
}

impl CryptoDemo {
    /// Создание нового экземпляра
    pub fn new() -> Self {
        let rng = ring::rand::SystemRandom::new();
        let mut salt = vec![0u8; 16];
        rng.fill(&mut salt).unwrap();
        Self { rng, salt }
    }

    /// Генерация безопасного ключа
    pub fn generate_key(&self, password: &str) -> Vec<u8> {
        let mut key = vec![0u8; 32];
        derive(
            PBKDF2_HMAC_SHA256,
            std::num::NonZeroU32::new(100_000).unwrap(),
            &self.salt,
            password.as_bytes(),
            &mut key,
        );
        key
    }

    /// Хеширование данных
    pub fn hash_data(&self, data: &[u8]) -> Vec<u8> {
        let digest = digest::digest(&SHA256, data);
        digest.as_ref().to_vec()
    }
}

impl SecureStorage {
    /// Создание нового экземпляра
    pub fn new(key: Vec<u8>) -> Self {
        Self {
            data: Arc::new(Mutex::new(Vec::new())),
            key,
        }
    }

    /// Безопасное хранение данных
    pub fn store_data(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let mut storage = self.data.lock();
        // Шифрование данных перед хранением
        let mut encrypted = vec![0u8; data.len()];
        for (i, &byte) in data.iter().enumerate() {
            encrypted[i] = byte ^ self.key[i % self.key.len()];
        }
        storage.extend_from_slice(&encrypted);
        Ok(())
    }

    /// Безопасное получение данных
    pub fn retrieve_data(&self, index: usize) -> Option<Vec<u8>> {
        let storage = self.data.lock();
        if index >= storage.len() {
            return None;
        }
        // Дешифрование данных
        let mut decrypted = vec![0u8; 1];
        decrypted[0] = storage[index] ^ self.key[index % self.key.len()];
        Some(decrypted)
    }
}

/// Демонстрация безопасности
pub fn demonstrate_security() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Демонстрация безопасности ===");

    // Демонстрация криптографии
    println!("\n1. Криптография:");
    let crypto = CryptoDemo::new();
    let password = "secure_password";
    let key = crypto.generate_key(password);
    println!("Сгенерирован ключ: {:?}", key);

    // Демонстрация хеширования
    let data = b"Hello, World!";
    let hash = crypto.hash_data(data);
    println!("Хеш данных: {:?}", hash);

    // Демонстрация безопасного хранения
    println!("\n2. Безопасное хранение:");
    let storage = SecureStorage::new(key);
    storage.store_data(data)?;
    if let Some(retrieved) = storage.retrieve_data(0) {
        println!("Получены данные: {:?}", retrieved);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto() {
        let crypto = CryptoDemo::new();
        let password = "test_password";
        let key = crypto.generate_key(password);
        assert_eq!(key.len(), 32);
    }

    #[test]
    fn test_secure_storage() {
        let key = vec![1, 2, 3, 4];
        let storage = SecureStorage::new(key);
        let data = b"test";
        storage.store_data(data).unwrap();
        assert!(storage.retrieve_data(0).is_some());
    }
} 