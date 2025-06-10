use crate::config::Config;
use anyhow::Result;
use regex::RegexBuilder;
use std::fs;

const NUMBER_OF_COLUMNS: i32 = 12;
// const NUMBER_OF_ROWS: i32 = 3;
// const IS_SPLIT: bool = true;
// const NUMBER_OF_THUMB_KEYS: i32 = 6;
// const KEY_DISPLAY_CHAR_WIDTH: i32 = 7;
// const TOTAL_NON_THUMB_KEYS: i32 = NUMBER_OF_COLUMNS * NUMBER_OF_ROWS;
// const TOTAL_KEYS: i32 = TOTAL_NON_THUMB_KEYS + NUMBER_OF_THUMB_KEYS;

pub fn parse_keymap(local_config: Config) -> Result<Vec<Vec<String>>> {
    let reg_exp = RegexBuilder::new(r"keymaps.*MATRIX.*\{(.*)\}")
        .multi_line(true)
        .dot_matches_new_line(true)
        .build()
        .unwrap();

    let contents = fs::read_to_string(&local_config.keymap_path)?;
    let caps = reg_exp.captures(&contents).unwrap();
    let inner = caps.get(1).unwrap().as_str();

    let reg_exp_inner = RegexBuilder::new(r"\[\d+\](.*?)\s\),")
        .multi_line(true)
        .dot_matches_new_line(true)
        .build()
        .unwrap();

    let reg_exp_layer = RegexBuilder::new(r"\w+\([^)]*\)|\w+")
        .multi_line(true)
        .build()
        .unwrap();

    let layers: Vec<String> = reg_exp_inner
        .find_iter(inner)
        .map(|m| m.as_str().to_string())
        .collect();

    let layers: Vec<String> = layers
        .into_iter()
        .map(|layer| {
            layer
                .lines()
                .filter(|line| !line.trim_start().starts_with("//"))
                .filter(|line| {
                    !line.trim_start().starts_with(")") && !line.trim_start().ends_with("(")
                })
                .collect::<Vec<&str>>()
                .join("\n")
        })
        .collect();

    let layers_vec: Vec<Vec<String>> = layers.iter().map(|layer| {
        reg_exp_layer.find_iter(layer)
            .map(|m| m.as_str().to_string())
            .collect()
    }).collect();

    Ok(layers_vec)
}

pub fn parse_full_keymap(local_config: Config) -> Result<Vec<Vec<Vec<String>>>> {
    let keymap = parse_keymap(local_config)?;
    let mut full_keymap = Vec::new();

    for layer in keymap {
        let layer_chunks = layer.chunks(NUMBER_OF_COLUMNS as usize)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<Vec<String>>>();
        full_keymap.push(layer_chunks);
    }

    println!("{:?}", full_keymap);

    Ok(full_keymap)
}
