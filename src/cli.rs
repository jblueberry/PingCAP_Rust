use clap::{Args, Parser, Subcommand};
use serde::{Serialize, Deserialize};

#[derive(Args)]
#[derive(Serialize, Deserialize)]
pub struct Set {
    key: String,
    value: String,
}

// generate new method for Set
impl Set {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}

#[derive(Args)]
#[derive(Serialize, Deserialize)]
pub struct Get {
    key: String,
}

// generate new method for Get
impl Get {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

#[derive(Args)]
#[derive(Serialize, Deserialize)]
pub struct Remove {
    key: String,
}

// generate new method for Remove
impl Remove {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

#[derive(Subcommand)]
#[derive(Serialize, Deserialize)]
pub enum Commands {
    Set(Set),
    Get(Get),
    Rm(Remove),
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}