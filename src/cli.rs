use clap::{Args, Parser, Subcommand};
use serde::{Serialize, Deserialize};

#[derive(Args)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Set {
    pub key: String,
    pub value: String,
}

// generate new method for Set
impl Set {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}

#[derive(Args)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Get {
    pub key: String,
}

// generate new method for Get
impl Get {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

#[derive(Args)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Remove {
    pub key: String,
}

// generate new method for Remove
impl Remove {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

#[derive(Subcommand)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Commands {
    Set(Set),
    Get(Get),
    Rm(Remove),
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct ClientCli {
    #[arg(long, default_value_t = String::from("127.0.0.1:4000"))]
    pub addr: String,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct ServerCli {
    #[arg(long, default_value_t = String::from("127.0.0.1:4000"))]
    pub addr: String,
    #[arg(long, default_value_t = String::from("kvs"))]
    pub engine: String,
}