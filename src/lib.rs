use std::{collections::HashMap, path::{Path, PathBuf}};

mod err;
use cli::Set;
pub use err::{KvsError, Result};

mod cli;
pub use cli::{Cli, Commands};

pub struct KvStore {
    path: PathBuf,
    map: HashMap<String, String>,
}

impl KvStore {
    pub fn open(path: &Path) -> Result<Self> {
        // convert path reference to owned path
        let path = path.to_owned();
        Ok(KvStore {
            path,
            map: HashMap::new(),
        })
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.map.insert(key.clone(), value.clone());
        let set_log = Set::new(key, value);
        let set_log_json = serde_json::to_string(&set_log)?;
        Ok(())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        self.map.remove(&key);
        Ok(())
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        if self.map.contains_key(&key) {
            let value = self.map.get(&key).unwrap();
            Ok(Some((*value).clone()))
        } else {
            Err(KvsError::KeyNotFound)
        }
    }
}
