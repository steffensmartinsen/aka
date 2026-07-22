use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "aka", about = "Manage shell aliases for WSL and PowerShell")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add or overwrite an alias
    Add {
        name: String,
        command: String,
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Remove an alias
    Remove { name: String },
    /// List aliases, optionally filtering by a search term
    List {
        #[arg(short, long)]
        search: Option<String>,
    },
    /// Regenerate shell files from the current alias store
    Generate,
    /// Import aliases from a profile file
    Import {
        /// Path to the profile JSON file
        file: String,
        /// On name conflict, let the imported alias win
        #[arg(long, conflicts_with = "keep")]
        overwrite: bool,
        /// On name conflict, keep the existing alias
        #[arg(long)]
        keep: bool,
    },
    /// Export current aliases to a profile file
    Export {
        /// Path to write the profile JSON to
        file: String,
    },
}