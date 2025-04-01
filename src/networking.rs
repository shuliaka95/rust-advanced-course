//! Демонстрация сетевого программирования в Rust
//! 
//! Этот модуль показывает основные концепции:
//! - HTTP сервер
//! - WebSocket
//! - UDP
//! - TCP
//! - Сетевые протоколы

use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;

// Простой TCP сервер
async fn tcp_server() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("TCP сервер запущен на 127.0.0.1:8080");

    while let Ok((mut socket, addr)) = listener.accept().await {
        println!("Новое подключение: {}", addr);
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Ошибка чтения: {}", e);
                        return;
                    }
                };

                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("Ошибка записи: {}", e);
                    return;
                }
            }
        });
    }

    Ok(())
}

// Простой UDP сервер
async fn udp_server() -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind("127.0.0.1:8081").await?;
    println!("UDP сервер запущен на 127.0.0.1:8081");

    let mut buf = [0; 1024];
    loop {
        let (n, addr) = socket.recv_from(&mut buf).await?;
        println!("Получено {} байт от {}", n, addr);

        if let Err(e) = socket.send_to(&buf[0..n], addr).await {
            eprintln!("Ошибка отправки: {}", e);
        }
    }
}

// Простой TCP клиент
async fn tcp_client() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("Подключено к серверу");

    stream.write_all(b"Привет, сервер!").await?;
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await?;
    println!("Получено: {}", String::from_utf8_lossy(&buf[0..n]));

    Ok(())
}

// Простой UDP клиент
async fn udp_client() -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind("127.0.0.1:0").await?;
    println!("UDP клиент запущен");

    socket.send_to(b"Привет, UDP сервер!", "127.0.0.1:8081").await?;
    let mut buf = [0; 1024];
    let (n, _) = socket.recv_from(&mut buf).await?;
    println!("Получено: {}", String::from_utf8_lossy(&buf[0..n]));

    Ok(())
}

pub async fn demonstrate_networking() {
    println!("\n1. Демонстрация TCP сервера и клиента:");
    let server_handle = tokio::spawn(tcp_server());
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    let client_handle = tokio::spawn(tcp_client());

    if let Err(e) = client_handle.await? {
        eprintln!("Ошибка клиента: {}", e);
    }

    println!("\n2. Демонстрация UDP сервера и клиента:");
    let server_handle = tokio::spawn(udp_server());
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    let client_handle = tokio::spawn(udp_client());

    if let Err(e) = client_handle.await? {
        eprintln!("Ошибка клиента: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_tcp_connection() {
        let server_handle = tokio::spawn(tcp_server());
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        assert!(tcp_client().await.is_ok());
    }

    #[tokio::test]
    async fn test_udp_connection() {
        let server_handle = tokio::spawn(udp_server());
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        assert!(udp_client().await.is_ok());
    }
} 