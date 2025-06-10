use crate::config::Config;
use anyhow::Result;
use regex::RegexBuilder;
use std::fs;

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
