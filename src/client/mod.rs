use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Client running");
    let mut stream = TcpStream::connect("127.0.0.1:8081").await.expect("Failed to connect to server");

    println!("Connected to server");

    // get message from user via stdin
    let mut message = String::new();
    println!("Enter a message to send to the server:");
    std::io::stdin().read_line(&mut message).expect("Failed to read from stdin");
    stream.write_all(message.as_bytes()).await.expect("Failed to send message");

    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).await.expect("Failed to read response");
    let response = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Response: {}", response);

    Ok(())
}