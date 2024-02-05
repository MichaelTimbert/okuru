use clap::{Parser,Subcommand};

#[derive(Parser, Debug)]
struct Cli {
    // command
    #[command(subcommand)]
    cmd: Commands,
}


#[derive(Subcommand, Debug)]
enum Commands {
    Send {
        file: std::path::PathBuf,
    },
    
    Receive {
        code: String,
    },
}

fn main() {
    println!("okuru");

    let args = Cli::parse();

    println!("{args:?}");

    match args.cmd {
        Commands::Send { file } => {
            todo!();
        }
        Commands::Receive { code } => {
            todo!();
        }
    }
    
}