mod cli;
mod generator;
mod models;
mod store;
mod update;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use models::Alias;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut store = store::load()?;

    match cli.command {
        Commands::Add {
            name,
            command,
            description,
        } => {
            store.aliases.insert(
                name.clone(),
                Alias {
                    command,
                    description,
                },
            );
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
            let aliases: Vec<_> = store
                .aliases
                .iter()
                .filter(|(name, alias)| {
                    search
                        .as_ref()
                        .map_or(true, |q| name.contains(q) || alias.command.contains(q))
                })
                .collect();

            if aliases.is_empty() {
                println!("No aliases found.");
            } else {
                for (name, alias) in aliases {
                    match &alias.description {
                        Some(desc) => println!("{:15} = {}  # {}", name, alias.command, desc),
                        None => println!("{:15} = {}", name, alias.command),
                    }
                }
            }
        }
        Commands::Generate => {
            generator::generate(&store)?;
        }
        Commands::Import {
            file,
            overwrite,
            keep,
        } => {
            let path = std::path::PathBuf::from(&file);
            let incoming = store::load_from(&path)?;

            let conflicts: Vec<&String> = incoming
                .aliases
                .keys()
                .filter(|name| store.aliases.contains_key(*name))
                .collect();

            if !conflicts.is_empty() && !overwrite && !keep {
                eprintln!("Conflict: these aliases already exist:");
                for name in &conflicts {
                    eprintln!("  {}", name);
                }
                eprintln!("\nRe-run with --overwrite (imported wins) or --keep (existing wins).");
                anyhow::bail!("import aborted due to {} conflict(s)", conflicts.len());
            }

            let mut added = 0;
            let mut updated = 0;
            for (name, alias) in incoming.aliases {
                let exists = store.aliases.contains_key(&name);
                if exists && keep {
                    continue;
                }
                if exists {
                    updated += 1;
                } else {
                    added += 1;
                }
                store.aliases.insert(name, alias);
            }

            if added == 0 && updated == 0 {
                println!("Nothing to import — no changes made.");
            } else {
                store::save(&store)?;
                generator::generate(&store)?;
                println!(
                    "Imported from {}: {} added, {} updated.",
                    file, added, updated
                );
                println!("Run 'source ~/.bashrc' to use them in this shell.");
            }
        }
        Commands::Export { file } => {
            let path = std::path::PathBuf::from(&file);
            store::save_to(&store, &path)?;
            println!("Exported {} alias(es) to {}", store.aliases.len(), file);
        }
        Commands::Update => {
            update::run()?;
        }
    }

    Ok(())
}
