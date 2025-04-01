//! Модуль для демонстрации сетевого программирования в Rust
//! 
//! Этот модуль показывает различные аспекты сетевого программирования:
//! - HTTP клиент/сервер
//! - WebSocket
//! - TCP/UDP
//! - Асинхронные сетевые операции

use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;
use std::net::SocketAddr;
use tokio::time::{timeout, Duration};

/// Реализация HTTP сервера
pub struct HttpServer {
    addr: SocketAddr,
}

impl HttpServer {
    /// Создание нового HTTP сервера
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }

    /// Запуск сервера
    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(self.addr).await?;
        println!("HTTP сервер запущен на {}", self.addr);

        loop {
            let (socket, addr) = listener.accept().await?;
            println!("Новое подключение от {}", addr);
            
            tokio::spawn(async move {
                if let Err(e) = handle_connection(socket).await {
                    eprintln!("Ошибка обработки соединения: {}", e);
                }
            });
        }
    }
}

/// Реализация WebSocket клиента
pub struct WebSocketClient {
    addr: SocketAddr,
}

impl WebSocketClient {
    /// Создание нового WebSocket клиента
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }

    /// Подключение к серверу
    pub async fn connect(&self) -> Result<TcpStream, Box<dyn Error>> {
        let stream = TcpStream::connect(self.addr).await?;
        println!("Подключено к WebSocket серверу на {}", self.addr);
        Ok(stream)
    }

    /// Отправка сообщения
    pub async fn send_message(&self, stream: &mut TcpStream, message: &str) -> Result<(), Box<dyn Error>> {
        stream.write_all(message.as_bytes()).await?;
        Ok(())
    }

    /// Получение сообщения
    pub async fn receive_message(&self, stream: &mut TcpStream) -> Result<String, Box<dyn Error>> {
        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer).await?;
        Ok(String::from_utf8_lossy(&buffer[..n]).to_string())
    }
}

/// Реализация UDP сервера
pub struct UdpServer {
    addr: SocketAddr,
}

impl UdpServer {
    /// Создание нового UDP сервера
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }

    /// Запуск сервера
    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let socket = UdpSocket::bind(self.addr).await?;
        println!("UDP сервер запущен на {}", self.addr);

        let mut buf = [0; 1024];
        loop {
            let (size, addr) = socket.recv_from(&mut buf).await?;
            println!("Получено {} байт от {}", size, addr);

            // Эхо-ответ
            socket.send_to(&buf[..size], addr).await?;
        }
    }
}

/// Обработка HTTP соединения
async fn handle_connection(mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];
    let n = socket.read(&mut buffer).await?;
    
    let request = String::from_utf8_lossy(&buffer[..n]);
    println!("Получен запрос:\n{}", request);

    // Простой HTTP ответ
    let response = "HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nHello, World!";
    socket.write_all(response.as_bytes()).await?;

    Ok(())
}

/// Демонстрация HTTP сервера
pub async fn demonstrate_http_server() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8080".parse()?;
    let server = HttpServer::new(addr);
    server.run().await
}

/// Демонстрация WebSocket клиента
pub async fn demonstrate_websocket_client() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8081".parse()?;
    let client = WebSocketClient::new(addr);
    let mut stream = client.connect().await?;

    client.send_message(&mut stream, "Hello, WebSocket!").await?;
    let response = client.receive_message(&mut stream).await?;
    println!("Получен ответ: {}", response);

    Ok(())
}

/// Демонстрация UDP сервера
pub async fn demonstrate_udp_server() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8082".parse()?;
    let server = UdpServer::new(addr);
    server.run().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_http_server() {
        let addr = "127.0.0.1:8083".parse().unwrap();
        let server = HttpServer::new(addr);
        
        // Запускаем сервер в отдельном потоке
        let server_handle = tokio::spawn(async move {
            if let Err(e) = server.run().await {
                eprintln!("Ошибка сервера: {}", e);
            }
        });
        
        // Даем серверу время на запуск
        sleep(Duration::from_millis(100)).await;
        
        // Проверяем, что сервер запустился
        assert!(!server_handle.is_finished());
        
        // Отменяем сервер
        server_handle.abort();
    }

    #[tokio::test]
    async fn test_websocket_client() {
        let addr = "127.0.0.1:8084".parse().unwrap();
        let client = WebSocketClient::new(addr);
        
        // Проверяем, что клиент не может подключиться к несуществующему серверу
        let result = client.connect().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_udp_server() {
        let addr = "127.0.0.1:8085".parse().unwrap();
        let server = UdpServer::new(addr);
        
        // Запускаем сервер в отдельном потоке
        let server_handle = tokio::spawn(async move {
            if let Err(e) = server.run().await {
                eprintln!("Ошибка сервера: {}", e);
            }
        });
        
        // Даем серверу время на запуск
        sleep(Duration::from_millis(100)).await;
        
        // Проверяем, что сервер запустился
        assert!(!server_handle.is_finished());
        
        // Отменяем сервер
        server_handle.abort();
    }
} 