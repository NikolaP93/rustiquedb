use std::sync::Arc;
use tokio::io::{ AsyncReadExt, AsyncWriteExt };
use store::KeyValueStore;

mod store;

enum Command {
    Get(String),
    Set(String, String),
    Delete(String),
}

async fn handle_connection(mut socket: tokio::net::TcpStream, kv_store: Arc<KeyValueStore>) {
    let mut buffer = [0; 1024];
    loop {
        match socket.read(&mut buffer).await {
            Ok(0) => {
                println!("Client closed the connection");
                break;
            }
            Ok(n) => {
                let received = String::from_utf8_lossy(&buffer[..n]).to_string();
                let parts: Vec<&str> = received.split_whitespace().collect();
                let command = match parts.as_slice() {
                    ["GET", key] => Command::Get(key.to_string()),
                    ["SET", key, value] => Command::Set(key.to_string(), value.to_string()),
                    ["DELETE", key] => Command::Delete(key.to_string()),
                    _ => {
                        if
                            let Err(e) = socket.write_all(
                                "Unknown command or wrong format".as_bytes()
                            ).await
                        {
                            println!("Failed to send response: {}", e);
                        }
                        continue;
                    }
                };

                let response = match command {
                    Command::Get(key) => kv_store.get(key.to_string()).await,
                    Command::Set(key, value) => {
                        kv_store.set(key.to_string(), value.to_string()).await;
                        kv_store.save("data.json").await.unwrap();
                        Some("Value set successfully".to_string())
                    }
                    Command::Delete(key) => {
                        kv_store.delete(key.to_string()).await;
                        Some("Key deleted successfully".to_string())
                    }
                };

                if let Some(response) = response {
                    if let Err(e) = socket.write_all(response.as_bytes()).await {
                        println!("Failed to send response: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Failed to read from socket: {}", e);
                break;
            }
        }
        buffer.fill(0); // Clear the buffer to prevent mixing messages
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8081").await?;
    println!("Server running on port 8081");

    let initial_store = match KeyValueStore::load("data.json").await {
        Ok(store) => store,
        Err(_) => {
            println!("No data file found, starting with empty store");
            KeyValueStore::new()
        }
    };

    let kv_store = Arc::new(initial_store);

    loop {
        let (socket, _) = listener.accept().await?;
        let kv_store = kv_store.clone();

        tokio::spawn(handle_connection(socket, kv_store));
    }
}
