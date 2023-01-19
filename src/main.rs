use std::process::exit;

use clap::{Args, Parser, Subcommand};

#[derive(Args)]
struct Set {
    key: String,
    value: String,
}

#[derive(Args)]
struct Get {
    key: String,
}

#[derive(Args)]
struct Remove {
    key: String,
}

#[derive(Subcommand)]
enum Commands {
    Set(Set),
    Get(Get),
    Rm(Remove),
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
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
}
