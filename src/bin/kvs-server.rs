#[macro_use]
extern crate log;

use clap::Parser;
use kvs::Result;
use kvs::ServerCli;
use std::net::SocketAddr;

// get the version from Cargo.toml
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

enum Engine {
    Kvs,
    Sled,
}

impl Engine {
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "kvs" => Ok(Engine::Kvs),
            "sled" => Ok(Engine::Sled),
            _ => Err(kvs::KvsError::UnknownEngine),
        }
    }
}

impl std::fmt::Display for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Engine::Kvs => write!(f, "kvs"),
            Engine::Sled => write!(f, "sled"),
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();
    // let cli = ServerCli::parse();
    info!("The version of the lib: {}", VERSION);

    let cli = ServerCli::parse();

    let engine = Engine::from_str(&cli.engine)?;
    let addr: SocketAddr = cli.addr.parse()?;

    info!("The addr of the server: {}", &addr);
    info!("The engine of the server: {}", &engine);

    // let mut store = match engine {
    //     Engine::Kvs => kvs::KvStore::open(std::env::current_dir()?)?,
    //     Engine::Sled => kvs::SledKvsEngine::open(std::env::current_dir()?)?,
    // };

    Ok(())
}
