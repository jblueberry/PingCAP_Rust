use clap::Parser;
use kvs::Result;
use std::process::exit;
// use src/cli.rs, here is src/bin/kvs.rs
use kvs::{Cli, Commands};

fn main() -> Result<()> {
    // get the current working directory
    let cwd = std::env::current_dir()?;
    // println!("current working directory: {}", cwd.display());
    let cli = Cli::parse();

    let mut store = kvs::KvStore::open(cwd)?;
    store.debug_print();

    // print cli.command
    // println!("cli.command: {:?}", cli.command);

    match cli.command {
        Commands::Set(_set_command) => store.set(_set_command.key, _set_command.value),
        Commands::Get(_get_command) => {
            let value = store.get(_get_command.key);
            match value {
                Ok(value) => {
                    match value {
                        Some(value) => println!("{}", value),
                        None => println!("Key not found"),
                    }
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                    exit(1);
                }
            }
            Ok(())
        }
        Commands::Rm(_rm_command) => {
            let value = store.remove(_rm_command.key);
            match value {
                Ok(_) => {Ok(())}
                Err(err) => match err {
                    kvs::KvsError::KeyNotFound => {
                        println!("Key not found");
                        exit(1);
                    }
                    _ => {
                        eprintln!("Error: {}", err);
                        exit(1);
                    }
                },
            }
        }
    }
}
