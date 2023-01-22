use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, Read, Seek},
    path::Path,
};

mod err;
pub use err::{KvsError, Result};

mod cli;
pub use cli::{Cli, Commands};

mod serialize;

pub struct KvStore {
    file: File,
    map: HashMap<String, usize>,
}

impl KvStore {
    pub fn debug_print(&self) {
        eprintln!("map: {:?}", self.map);
    }

    pub fn new(file: File) -> Self {
        // borrow the file and read line by line
        let mut reader = std::io::BufReader::new(&file);
        let mut map = HashMap::new();
        let mut position = 0;
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
        }

        KvStore { file, map: map }
    }

    pub fn open<P: AsRef<Path>>(dir_path: P) -> Result<Self> {
        // convert path reference to owned path
        let dir_path = String::from(dir_path.as_ref().to_str().unwrap());
        // println!("open for {}", dir_path);
        let path = dir_path + "/log.log";

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
        Ok(Self::new(file))
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let set_command = Commands::Set(cli::Set::new(key.clone(), value.clone()));
        self.map
            .insert(key, serialize::append_to_file(&mut self.file, set_command)?);
        Ok(())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        if self.map.contains_key(&key) {
            let rm_command = Commands::Rm(cli::Remove::new(key.clone()));
            serialize::append_to_file(&mut self.file, rm_command)?;
            self.map.remove(&key);
            Ok(())
        } else {
            Err(KvsError::KeyNotFound)
        }
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        // iterate the map
        // for (k, v) in &self.map {
        //     eprintln!("{}: {}", k, v);
        // }
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
