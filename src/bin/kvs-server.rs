#[macro_use]
extern crate log;

use clap::Parser;
use kvs::ServerCli;

// get the version from Cargo.toml
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
fn main() {
    env_logger::init();
    // let cli = ServerCli::parse();
    info!("The version of the lib: {}", VERSION);

    let cli = ServerCli::parse();
    let engine = cli.engine;
    let addr = cli.addr;
    info!("The addr of the server: {}", &addr);
    info!("The engine of the server: {}", &engine);
}