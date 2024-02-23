
use super::command::{ parse_command, Command };
use super::store::KeyValueStore;
use std::sync::Arc;
use tokio::io::{ AsyncReadExt, AsyncWriteExt };


pub async fn handle_connection(mut socket: tokio::net::TcpStream, kv_store: Arc<KeyValueStore>) {
  let mut buffer = [0; 1024];
  loop {
      match socket.read(&mut buffer).await {
          Ok(0) => {
              println!("Client closed the connection");
              break;
          }
          Ok(n) => {
              let received = String::from_utf8_lossy(&buffer[..n]).to_string();
              let command = parse_command(&received);

              let response = match command {
                  Some(Command::Get(key)) => kv_store.get(key.to_string()).await,
                  Some(Command::Set(key, value)) => {
                      kv_store.set(key.to_string(), value.to_string()).await;
                      kv_store.save("data.json").await.unwrap();
                      Some("Value set successfully".to_string())
                  }
                  Some(Command::Delete(key)) => {
                      kv_store.delete(key.to_string()).await;
                      Some("Key deleted successfully".to_string())
                  }
                  None => Some("Invalid command".to_string()),
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