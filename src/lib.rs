use std::{collections::HashMap, fs::File, io::BufRead, path::Path};

mod err;
use cli::Set;
pub use err::{KvsError, Result};

mod cli;
pub use cli::{Cli, Commands};

mod serialize;

pub struct KvStore {
    file: File,
    map: HashMap<String, usize>,
    log_size: usize,
    path: String,
}

impl KvStore {
    pub fn compact(&mut self) {
        // eprintln!("compact called");
        let size = self.map.len();
        if self.log_size > size {
            eprintln!("going to compact");
            // create a new file
            let mut new_file = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .open(self.path.clone() + "/log_new.log")
                .unwrap();
            let mut new_map = HashMap::new();
            // iterating the map
            for (key, value) in &self.map {
                let command = serialize::read_from_file(&mut self.file, *value).unwrap();
                // write this command into the new file
                let pos = serialize::append_to_file(&mut new_file, command).unwrap();

                new_map.insert(key.clone(), pos);
            }
            
            self.map = new_map;
            self.file = new_file;
            // delete the old file
            std::fs::remove_file(self.path.clone() + "/log.log").unwrap();
            // rename the new file
            std::fs::rename(self.path.clone() + "/log_new.log", self.path.clone() + "/log.log").unwrap();

            self.log_size = size;
        }
    }
    pub fn debug_print(&self) {
        eprintln!("map: {:?}", self.map);
    }

    pub fn new(file: File, dir_path: String) -> Self {
        // borrow the file and read line by line
        let mut reader = std::io::BufReader::new(&file);
        let mut map = HashMap::new();
        let mut position = 0;
        let mut size = 0;
        loop {
            let mut buf = String::new();
            let line = reader.read_line(&mut buf);
            if line.is_err() {
                break;
            }
            let line = line.unwrap();
            if line == 0 {
                break;
            }
            // print the buf
            // eprintln!("buf:\n{}", buf);
            let command: Commands = serde_json::from_str(&buf).unwrap();
            match command {
                Commands::Set(set) => {
                    map.insert(set.key, position);
                }
                Commands::Rm(rm) => {
                    map.remove(&rm.key);
                }
                _ => {}
            }
            position += line;
            size += 1;
        }

        KvStore {
            file,
            map: map,
            log_size: size,
            path: dir_path
        }
    }

    pub fn open<P: AsRef<Path>>(dir_path: P) -> Result<Self> {
        // convert path reference to owned path
        let dir_path = String::from(dir_path.as_ref().to_str().unwrap());
        // println!("open for {}", dir_path);
        let path = dir_path.clone() + "/log.log";

        // eprintln!("going to open file: {}", path);

        let cwd = std::env::current_dir()?;
        // eprintln!("current working directory: {}", cwd.display());

        // open the log.log file under the given path

        // open a file in read-write mode
        let file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(path)?;

        // read the file content
        // let mut content = String::new();
        // file.read_to_string(&mut content);
        // eprintln!("content: {}", content);

        // println!("11");
        Ok(Self::new(file, dir_path))
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let set_command = Commands::Set(cli::Set::new(key.clone(), value.clone()));
        self.map
            .insert(key, serialize::append_to_file(&mut self.file, set_command)?);
        self.log_size += 1;
        self.compact();
        Ok(())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        eprintln!("remove called");
        if self.map.contains_key(&key) {
            let rm_command = Commands::Rm(cli::Remove::new(key.clone()));
            serialize::append_to_file(&mut self.file, rm_command)?;
            self.log_size += 1;
            self.map.remove(&key);
            self.compact();
            Ok(())
        } else {
            Err(KvsError::KeyNotFound)
        }
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        eprintln!("get called");
        if self.map.contains_key(&key) {
            let command = serialize::read_from_file(&mut self.file, self.map[&key])?;
            match command {
                Commands::Set(set) => Ok(Some(set.value)),
                _ => Err(KvsError::KeyNotFound),
            }
        } else {
            Ok(None)
        }
    }
}
