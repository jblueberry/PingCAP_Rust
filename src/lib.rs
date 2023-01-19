use std::collections::HashMap;
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        KvStore {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }

    pub fn get(&self, key: String) -> Option<String> {
        if self.map.contains_key(&key) {
            let value = self.map.get(&key).unwrap();
            Some((*value).clone())
        } else {
            None
        }
    }
}
