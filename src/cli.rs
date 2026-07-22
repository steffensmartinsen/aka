use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "alias-mgr", about = "Manage shell aliases for WSL and PowerShell")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        name: String,
        command: String,
        #[arg(short, long)]
        description: Option<String>,
    },
    Remove { name: String },
    List {
        #[arg(short, long)]
        search: Option<String>,
    },
    Generate,
}