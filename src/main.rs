use anyhow::{Result};
use clap::{Parser, Subcommand};
use dialoguer::Input;
use regex::RegexBuilder;
use std::fs;
use std::path::PathBuf;
mod config;
mod render;
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

fn load_keymap_dictonary() -> Result<render::KeymapDictionary> {
    let keymap_dictionary: render::KeymapDictionary = serde_json::from_str(KEYCODES_JSON)?;
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

            let mut layer_index = 0;
            for part in layers {
                render::render_layer(&part, &keymap_dict, layer_index);
                layer_index = layer_index + 1;
            }
        }
        Commands::Open => {
            let local_config = config::load_config()?;
            let keymap_dict = load_keymap_dictonary()?;

            let parser_config = config::Config {
                keymap_path: local_config.keymap_path,
            };
            let layers = keymap_parser::parse_keymap(parser_config)?;

            let rendered_text = render::render_all_layers(&layers, &keymap_dict);
            render::open_in_window(rendered_text).map_err(|e| anyhow::anyhow!("Failed to open window: {}", e))?;
        }
    }

    Ok(())
}

