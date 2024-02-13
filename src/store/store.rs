use std::collections::HashMap;
use std::sync::RwLock;

pub struct KeyValueStore {
    store: RwLock<HashMap<String, String>>,
}

impl KeyValueStore {
    pub fn new() -> KeyValueStore {
        KeyValueStore {
            store: RwLock::new(HashMap::new()),
        }
    }

    pub fn get(&self, key: String) -> Option<String> {
        let read_access = self.store.read().unwrap();
        read_access.get(&key).cloned()
    }

    pub fn set(&self, key: String, value: String) {
        let mut write_access = self.store.write().unwrap();
        write_access.insert(key, value);
    }

    pub fn delete(&self, key: String) {
        let mut write_access = self.store.write().unwrap();
        write_access.remove(&key);
    }
}
