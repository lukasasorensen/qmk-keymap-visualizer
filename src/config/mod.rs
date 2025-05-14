use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub keymap_path: PathBuf,
}

pub fn get_config_path() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "qmk", "keymap-visualizer")
        .context("Could not determine project directory")?;
    let config_dir = proj_dirs.config_dir();
    fs::create_dir_all(config_dir)?;
    Ok(config_dir.join("config.json"))
}

pub fn load_config() -> Result<Config> {
    let config_path = get_config_path()?;
    if !config_path.exists() {
        return Err(anyhow::anyhow!(
            "Configuration not found. Please run 'setup' first."
        ));
    }
    let config_str = fs::read_to_string(config_path)?;
    Ok(serde_json::from_str(&config_str)?)
}

pub fn save_config(config_json: &Config) -> Result<()> {
    let config_path = get_config_path()?;
    let config_str = serde_json::to_string_pretty(config_json)?;
    fs::write(config_path, config_str)?;
    Ok(())
}