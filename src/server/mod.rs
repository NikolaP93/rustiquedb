mod store;
mod command;
mod connection;

use std::sync::Arc;
use store::KeyValueStore;
use connection::handle_connection;

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
