mod cli;
mod generator;
mod models;
mod store;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use models::Alias;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut store = store::load()?;

    match cli.command {
        Commands::Add { name, command, description } => {
            store.aliases.insert(name.clone(), Alias { command, description });
            store::save(&store)?;
            generator::generate(&store)?;
            println!("Added alias '{}'", name);
            println!("Run 'source ~/.bashrc' to use it in this shell.");
        }
        Commands::Remove { name } => {
            if store.aliases.remove(&name).is_none() {
                anyhow::bail!("Alias '{}' not found", name);
            }
            store::save(&store)?;
            generator::generate(&store)?;
            println!("Removed alias '{}'", name);
            println!("Run 'source ~/.bashrc' to apply in this shell.");
        }
        Commands::List { search } => {
            let aliases: Vec<_> = store.aliases.iter()
                .filter(|(name, alias)| {
                    search.as_ref().map_or(true, |q| {
                        name.contains(q) || alias.command.contains(q)
                    })
                })
                .collect();

            if aliases.is_empty() {
                println!("No aliases found.");
            } else {
                for (name, alias) in aliases {
                    match &alias.description {
                        Some(desc) => println!("{:15} = {}  # {}", name, alias.command, desc),
                        None       => println!("{:15} = {}", name, alias.command),
                    }
                }
            }
        }
        Commands::Generate => {
            generator::generate(&store)?;
        }
    }

    Ok(())
}