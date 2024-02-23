use std::sync::Arc;
use tokio::io::{ AsyncReadExt, AsyncWriteExt };
use store::KeyValueStore;

mod store;

async fn handle_connection(mut socket: tokio::net::TcpStream, kv_store: Arc<KeyValueStore>) {
  let mut buffer = [0; 1024];
  loop {
      match socket.read(&mut buffer).await {
          Ok(0) => {
              println!("Client closed the connection");
              break;
          },
          Ok(n) => {
              let received = String::from_utf8_lossy(&buffer[..n]).to_string();
              let parts: Vec<&str> = received.split_whitespace().collect();
              let response = match parts.as_slice() {
                  ["GET", key] => kv_store.get(key.to_string()).await,
                  ["SET", key, value] => {
                      kv_store.set(key.to_string(), value.to_string()).await;
                      kv_store.save("data.json").await.unwrap();
                      Some("Value set successfully".to_string())
                  },
                  ["DELETE", key] => {
                      kv_store.delete(key.to_string()).await;
                      Some("Key deleted successfully".to_string())
                  },
                  _ => Some("Unknown command or wrong format".to_string()),
              };

              if let Some(response) = response {
                  if let Err(e) = socket.write_all(response.as_bytes()).await {
                      println!("Failed to send response: {}", e);
                  }
              }
          },
          Err(e) => {
              println!("Failed to read from socket: {}", e);
              break;
          }
      }
      buffer.fill(0); // Clear the buffer to prevent mixing messages
  }
}


pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let kv_store = Arc::new(KeyValueStore::new());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8081").await?;
    println!("Server running on port 8081");

    loop {
        let (socket, _) = listener.accept().await?;
        let kv_store = kv_store.clone();

        tokio::spawn(handle_connection(socket, kv_store));
    }
}
