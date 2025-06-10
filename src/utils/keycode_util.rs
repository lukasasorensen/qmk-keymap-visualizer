use regex::RegexBuilder;
use std::collections::HashMap;

pub type KeymapDictionary = HashMap<String, String>;

pub fn get_key_code_human_readable(keycode: &str, keymap_dictionary: &KeymapDictionary) -> String {
    let mut keycode_str = keycode.to_string();
    keycode_str = keycode_str.trim().to_string();
    if keycode_str.contains("(") {
        let keycode_regex = RegexBuilder::new(r"\w+\((.*)\)").build().unwrap();
        let caps = keycode_regex.captures(&keycode_str).unwrap();
        let inner = caps.get(1).unwrap().as_str();
        keycode_str = inner.to_string();
    }

    keycode_str = keycode_str.replace(",", "");

    let human_readable = keymap_dictionary.get(&keycode_str);
    if let Some(human_readable) = human_readable {
        let human_readable = human_readable.clone();
        human_readable
    } else {
        keycode_str
    }
}