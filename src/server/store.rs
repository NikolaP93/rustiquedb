use std::collections::HashMap;
use tokio::sync::RwLock;
use std::io::{ self };
use serde_json;

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
        let serialized = serde_json::to_string(&*read_access).unwrap();
        println!("Saving to file: {}", serialized);
        tokio::fs::write(filename, serialized).await?;
        Ok(())
    }

    pub async fn get(&self, key: String) -> Option<String> {
        let read_access = self.store.read().await;
        read_access.get(&key).cloned()
    }

    pub async fn set(&self, key: String, value: String) {
        let mut write_access = self.store.write().await;
        println!("Saving key-value pair: {} = {}", &key, &value);
        write_access.insert(key, value);
    }

    pub async fn delete(&self, key: String) {
        let mut write_access = self.store.write().await;
        write_access.remove(&key);
    }
}
