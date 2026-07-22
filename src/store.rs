use crate::models::{AliasStore};
use anyhow::{Context, Result};
use std::path::PathBuf;

pub fn store_path() -> Result<PathBuf> {
    let dir = dirs::config_dir()
        .context("Could not find config directory")?
        .join("aka");
    std::fs::create_dir_all(&dir)?;
    Ok(dir.join("aliases.json"))
}

pub fn load() -> Result<AliasStore> {
    let path = store_path()?;
    if !path.exists() {
        return Ok(AliasStore::default());
    }
    let contents = std::fs::read_to_string(&path)
        .context("Failed to read aliases.json")?;
    serde_json::from_str(&contents).context("Failed to parse aliases.json")
}

pub fn save(store: &AliasStore) -> Result<()> {
    let path = store_path()?;
    let contents = serde_json::to_string_pretty(store)?;
    std::fs::write(&path, contents).context("Failed to write aliases.json")
}