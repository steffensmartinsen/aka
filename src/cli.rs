use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "aka", version, about = "Manage shell aliases for Unix based terminals")]
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
    /// Update aka to the latest release
    Update,
    /// Install aka's shell integration into your shell's rc file
    Install {
        /// Force a shell instead of auto-detecting
        #[arg(long)]
        shell: Option<String>,
        /// Show what would change without writing
        #[arg(long)]
        dry_run: bool,
    },
    /// Remove aka completely: shell integration, config, and binary
    Uninstall {
        #[arg(long)]
        shell: Option<String>,
        #[arg(long)]
        dry_run: bool,
    },
}