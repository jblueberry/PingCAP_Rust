use clap::Parser;
use kvs::ServerCli;

fn main() {
    let cli = ServerCli::parse();
    println!("Hello, world!");
}