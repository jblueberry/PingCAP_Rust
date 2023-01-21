use std::process::exit;
use clap::Parser;
use kvs::Result;
// use src/cli.rs, here is src/bin/kvs.rs
use kvs::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Set(_set_command) => {
            eprintln!("unimplemented");
            exit(-1);
        }
        Commands::Get(_get_command) => {
            eprintln!("unimplemented");
            exit(-1);
        }
        Commands::Rm(_rm_command) => {
            eprintln!("unimplemented");
            exit(-1);
        }
    }

    Ok(())
}