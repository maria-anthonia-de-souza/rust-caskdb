use std::collections::HashMap;

pub struct MemoryStorage {
    store: HashMap<String, String>,
}
impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
    pub fn set(&mut self, key: String, val: String) {
        self.store.insert(key, val);
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.store.get(key).map(String::as_str)
    }
    pub fn close(&mut self) {}
}
