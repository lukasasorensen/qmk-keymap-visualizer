use anyhow::{Result};
use clap::{Parser, Subcommand};
use dialoguer::Input;
use std::path::PathBuf;
mod config;
mod ascii;
mod keymap_parser;

pub static KEYCODES_JSON: &str = include_str!("../data/keycodes.json");

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set up the path to your keymap.c file
    Setup,
    /// Show the contents of your keymap.c file
    Show,
    /// Open the keymap in a window
    Open,
}

fn load_keymap_dictonary() -> Result<ascii::KeymapDictionary> {
    let keymap_dictionary: ascii::KeymapDictionary = serde_json::from_str(KEYCODES_JSON)?;
    Ok(keymap_dictionary)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Setup => {
            println!("Welcome to QMK Keymap Visualizer Setup!");
            println!("Please enter the path to your keymap.c file:");

            let path: String = Input::new().with_prompt("Path").interact_text()?;

            let path = PathBuf::from(path);
            if !path.exists() {
                return Err(anyhow::anyhow!("The specified path does not exist"));
            }

            let local_config = config::Config { keymap_path: path };
            config::save_config(&local_config)?;
            println!("Configuration saved successfully!");
        }
        Commands::Show => {
            let local_config = config::load_config()?;
            let keymap_dict = load_keymap_dictonary()?;

            let parser_config = config::Config {
                keymap_path: local_config.keymap_path,
            };
            let layers = keymap_parser::parse_keymap(parser_config)?;

            let rendered_text = ascii::render_all_layers(&layers, &keymap_dict);
            println!("{}", rendered_text);

        }
        Commands::Open => {
            let local_config = config::load_config()?;
            let keymap_dict = load_keymap_dictonary()?;

            let parser_config = config::Config {
                keymap_path: local_config.keymap_path,
            };
            let layers = keymap_parser::parse_keymap(parser_config)?;

            let rendered_text = ascii::render_all_layers(&layers, &keymap_dict);
            ascii::open_in_window(rendered_text).map_err(|e| anyhow::anyhow!("Failed to open window: {}", e))?;
        }
    }

    Ok(())
}

