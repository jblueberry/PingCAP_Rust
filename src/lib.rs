use std::{collections::HashMap, path::Path};

mod err;
pub use err::{Result, KvsError};

pub struct KvStore {
    map: HashMap<String, String>,
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

impl KvStore {
    pub fn new() -> Self {
        KvStore {
            map: HashMap::new(),
        }
    }

    pub fn open(path: &Path) -> Result<Self> {
        unimplemented!()
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.map.insert(key, value);
        Ok(())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        self.map.remove(&key);
        Ok(())
    }

    pub fn get(&self, key: String) -> Result<Option<String>>{
        if self.map.contains_key(&key) {
            let value = self.map.get(&key).unwrap();
            Ok(Some((*value).clone()))
        } else {
            Err(KvsError::KeyNotFound)
        }
    }
}
