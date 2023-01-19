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
    Rm(Remove)
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
        Commands::Set(set_command) => {
            println!("{:?}", set_command.key);
        },
        other => {
            
        }
    }
}
