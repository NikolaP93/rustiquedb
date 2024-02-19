mod store;
use std;
use std::sync::Arc;

use store::KeyValueStore;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let kv_store = Arc::new(KeyValueStore::new());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8081").await?;

    println!("Server running on port 8081");

    loop {
        let kv_store = kv_store.clone();
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buffer = [0; 1024];

            match socket.read(&mut buffer).await {
                Ok(n) => {
                    let received = String::from_utf8_lossy(&buffer[..n]).to_string();
                    let parts: Vec<&str> = received.split_whitespace().collect();

                    let response = match parts.as_slice() {
                        ["GET", key] => kv_store
                            .get(key.to_string())
                            .map_or("Key not found".to_string(), |value| value),
                        ["SET", key, value] => {
                            kv_store.set(key.to_string(), value.to_string());
                            "Key set successfully".to_string()
                        }
                        ["DELETE", key] => {
                            kv_store.delete(key.to_string());
                            "Key deleted successfully".to_string()
                        }
                        _ => "Unknown command or wrong format".to_string(),
                    };

                    if let Err(e) = socket.write_all(response.as_bytes()).await {
                        println!("Failed to send response: {}", e);
                    }
                }
                Err(e) => println!("Failed to read from socket: {}", e),
            }
        });
    }
}
