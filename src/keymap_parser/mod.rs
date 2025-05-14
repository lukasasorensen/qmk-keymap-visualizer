use regex::RegexBuilder;
use std::fs;
use anyhow::Result;
use crate::config::Config;

pub fn parse_keymap(local_config: Config) -> Result<Vec<String>> {
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

    Ok(reg_exp_inner.find_iter(inner)
        .map(|m| m.as_str().to_string())
        .collect())
}
