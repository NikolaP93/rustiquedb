use std::collections::HashMap;
use tokio::sync::RwLock;
use std::io::{ self };
use serde_json;
use tokio::io::AsyncWriteExt;

pub struct KeyValueStore {
    store: RwLock<HashMap<String, String>>,
}

impl KeyValueStore {
    pub fn new() -> Self {
        KeyValueStore {
            store: RwLock::new(HashMap::new()),
        }
    }

    pub async fn load(filename: &str) -> io::Result<Self> {
        let file_content = tokio::fs::read_to_string(filename).await?;
        let store: HashMap<String, String> = serde_json::from_str(&file_content)?;
        Ok(KeyValueStore {
            store: RwLock::new(store),
        })
    }

    pub async fn save(&self, filename: &str) -> io::Result<()> {
        let read_access = self.store.read().await;
        let serialized = serde_json::to_string(&*read_access);
        match serialized {
            Ok(serialized) => {
                tokio::fs::write(filename, serialized).await?;
                Ok(())
            }
            Err(e) => {
                println!("Failed to serialize store: {}", e);
                Err(io::Error::new(io::ErrorKind::Other, "Failed to serialize store"))
            }
        }
    }

    pub async fn log_command(&self, command: &str) -> io::Result<()> {
      let mut aof = tokio::fs::OpenOptions::new().create(true).append(true).open("aof.txt").await?;
      aof.write_all(command.as_bytes()).await?;
      Ok(())
    }

    pub async fn get(&self, key: String) -> Option<String> {
      let read_access = self.store.read().await;
      self.log_command(&format!("GET {}\n", &key)).await.unwrap();
      read_access.get(&key).cloned()
    }

    pub async fn set(&self, key: String, value: String) {
        let mut write_access = self.store.write().await;
        println!("Saving key-value pair: {} = {}", &key, &value);
        self.log_command(&format!("SET {} {}\n", &key, &value)).await.unwrap();
        write_access.insert(key, value);
    }

    pub async fn delete(&self, key: String) {
        let mut write_access = self.store.write().await;
        self.log_command(&format!("DELETE {}\n", &key)).await.unwrap();
        write_access.remove(&key);
    }
}
