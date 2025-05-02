use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use dialoguer::Input;
use directories::ProjectDirs;
use regex::RegexBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

static KEYCODES_JSON: &str = include_str!("../data/keycodes.json");
const NUMBER_OF_COLUMNS: i32 = 12;
const NUMBER_OF_ROWS: i32 = 3;
const IS_SPLIT: bool = true;
const NUMBER_OF_THUMB_KEYS: i32 = 6;
const KEY_DISPLAY_CHAR_WIDTH: i32 = 7;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    keymap_path: PathBuf,
}

type KeymapDictionary = HashMap<String, String>;

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
}

fn get_config_path() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "qmk", "keymap-visualizer")
        .context("Could not determine project directory")?;
    let config_dir = proj_dirs.config_dir();
    fs::create_dir_all(config_dir)?;
    Ok(config_dir.join("config.json"))
}

fn load_config() -> Result<Config> {
    let config_path = get_config_path()?;
    if !config_path.exists() {
        return Err(anyhow::anyhow!(
            "Configuration not found. Please run 'setup' first."
        ));
    }
    let config_str = fs::read_to_string(config_path)?;
    Ok(serde_json::from_str(&config_str)?)
}

fn save_config(config: &Config) -> Result<()> {
    let config_path = get_config_path()?;
    let config_str = serde_json::to_string_pretty(config)?;
    fs::write(config_path, config_str)?;
    Ok(())
}

fn load_keymap_dictonary() -> Result<KeymapDictionary> {
    let keymap_dictionary: KeymapDictionary = serde_json::from_str(KEYCODES_JSON)?;
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

            let config = Config { keymap_path: path };
            save_config(&config)?;
            println!("Configuration saved successfully!");
        }
        Commands::Show => {
            let reg_exp = RegexBuilder::new(r"keymaps.*MATRIX.*\{(.*)\}")
                .multi_line(true)
                .dot_matches_new_line(true)
                .build()
                .unwrap();

            let config = load_config()?;
            let keymap_dict = load_keymap_dictonary()?;
            let contents = fs::read_to_string(&config.keymap_path)?;
            let caps = reg_exp.captures(&contents).unwrap();
            let inner = caps.get(1).unwrap().as_str();

            let reg_exp_inner = RegexBuilder::new(r"\[\d+\](.*?)\s\),")
                .multi_line(true)
                .dot_matches_new_line(true)
                .build()
                .unwrap();

            let reg_exp_layer = RegexBuilder::new(r"\w+?,")
                .multi_line(true)
                .build()
                .unwrap();

            let mut layer_index = 0;
            for part in reg_exp_inner.find_iter(&inner) {
                println!("---- LAYER {} start ----", layer_index.to_string());
                let inner_str = part.as_str();

                layer_index = layer_index + 1;

                let mut keycode_index = 0;
                let mut row_index = 1;
                for keycode in reg_exp_layer.find_iter(&inner_str) {
                    if keycode_index == 0 {
                        print_dashes();
                    }
                    keycode_index = keycode_index + 1;
                    let is_end_row;
                    let is_split_gap;

                    let is_thumb_row = IS_SPLIT && row_index > NUMBER_OF_ROWS;

                    if is_thumb_row {
                        is_end_row = keycode_index % NUMBER_OF_THUMB_KEYS == 0;
                        is_split_gap =
                            !is_end_row && keycode_index % (NUMBER_OF_THUMB_KEYS / 2) == 0;
                    } else {
                        is_end_row = keycode_index % NUMBER_OF_COLUMNS == 0;
                        is_split_gap =
                            !is_end_row && IS_SPLIT && keycode_index % (NUMBER_OF_COLUMNS / 2) == 0;
                    }

                    let keycode_str = keycode.as_str();
                    let mut human_readable =
                        get_key_code_human_readable(&keycode_str, &keymap_dict);
                    human_readable = create_key_gui(&human_readable);
                    print!("{}", human_readable);

                    if is_split_gap {
                        print!("|      ")
                    }

                    if is_end_row {
                        println!("|");
                        print_dashes();
                        row_index = row_index + 1;
                    }
                }
                println!("");
                print_dashes();
                println!("---- LAYER end ----");
            }
        }
    }

    Ok(())
}

fn print_dashes() {
    let num_of_dashes = ((KEY_DISPLAY_CHAR_WIDTH + 1) * NUMBER_OF_COLUMNS) + 8;
    let mut dashes = String::new();
    for _ in 0..num_of_dashes {
        dashes.push('-');
    }
    println!("{}", dashes.to_string());
}

fn get_key_code_human_readable(keycode: &str, keymap_dictionary: &KeymapDictionary) -> String {
    let mut keycode_str = keycode.to_string();
    keycode_str.pop();
    let human_readable = keymap_dictionary.get(&keycode_str);
    if let Some(human_readable) = human_readable {
        let human_readable = human_readable.clone();
        human_readable
    } else {
        keycode_str
    }
}

fn create_key_gui(key_str: &str) -> String {
    let short_key = key_str
        .chars()
        .take(KEY_DISPLAY_CHAR_WIDTH as usize)
        .collect::<String>();
    let key_length = short_key.len();
    let mut s = String::new();
    let n_spaces = KEY_DISPLAY_CHAR_WIDTH - key_length as i32;
    let l_spaces = n_spaces / 2;
    let r_spaces = n_spaces - l_spaces;
    s.push('|');
    for _ in 0..l_spaces {
        s.push(' ')
    }
    s.push_str(&short_key);
    for _ in 0..r_spaces {
        s.push(' ')
    }
    s
}
