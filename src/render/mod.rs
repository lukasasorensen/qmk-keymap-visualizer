use regex::RegexBuilder;
use std::collections::HashMap;
use anyhow::Result;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};
use egui::{Context, Visuals};

const NUMBER_OF_COLUMNS: i32 = 12;
const NUMBER_OF_ROWS: i32 = 3;
const IS_SPLIT: bool = true;
const NUMBER_OF_THUMB_KEYS: i32 = 6;
const KEY_DISPLAY_CHAR_WIDTH: i32 = 7;

pub type KeymapDictionary = HashMap<String, String>;

pub fn render_layer(inner_str: &str, keymap_dict: &KeymapDictionary, layer_index: i32) {
    let reg_exp_layer = RegexBuilder::new(r"\w+?,")
        .multi_line(true)
        .build()
        .unwrap();

    println!("---- LAYER {} start ----", layer_index.to_string());

    let mut keycode_index = 1;
    let mut row_index = 1;
    for keycode in reg_exp_layer.find_iter(&inner_str) {
        let is_end_row = render_keycode(keycode.as_str(), keycode_index, row_index, &keymap_dict);
        if is_end_row {
            println!("|");
            print_dashes();
            row_index = row_index + 1;
        }
        keycode_index = keycode_index + 1;
    }
    println!("");
    print_dashes();
    println!("---- LAYER end ----");
}

fn render_keycode(
    keycode: &str,
    keycode_index: i32,
    row_index: i32,
    keymap_dict: &KeymapDictionary,
) -> bool {
    if keycode_index == 1 {
        print_dashes();
    }
    let is_end_row;
    let is_split_gap;

    let is_thumb_row = IS_SPLIT && row_index > NUMBER_OF_ROWS;

    if is_thumb_row {
        is_end_row = keycode_index % NUMBER_OF_THUMB_KEYS == 0;
        is_split_gap = !is_end_row && keycode_index % (NUMBER_OF_THUMB_KEYS / 2) == 0;
    } else {
        is_end_row = keycode_index % NUMBER_OF_COLUMNS == 0;
        is_split_gap = !is_end_row && IS_SPLIT && keycode_index % (NUMBER_OF_COLUMNS / 2) == 0;
    }

    let mut human_readable = get_key_code_human_readable(&keycode, &keymap_dict);
    human_readable = create_key_gui(&human_readable);
    print!("{}", human_readable);

    if is_split_gap {
        let repeat_count = KEY_DISPLAY_CHAR_WIDTH;
        print!("|{}", " ".repeat(repeat_count as usize));
    }
    return is_end_row;
}

fn print_dashes() {
    let column_width_with_pipes = KEY_DISPLAY_CHAR_WIDTH + 1;
    let num_of_dashes = (column_width_with_pipes * NUMBER_OF_COLUMNS) + 8;
    let line = (0..num_of_dashes)
        .map(|i| {
            if IS_SPLIT
                && i > (num_of_dashes / 2) - (column_width_with_pipes / 2)
                && i < (num_of_dashes / 2) + (column_width_with_pipes / 2)
            {
                ' '
            } else if i % column_width_with_pipes == 0 {
                '+'
            } else {
                '-'
            }
        })
        .collect::<String>();
    println!("{}", line);
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
    let key = key_str
        .chars()
        .take(KEY_DISPLAY_CHAR_WIDTH as usize)
        .collect::<String>();
    let padding = KEY_DISPLAY_CHAR_WIDTH - key.len() as i32;
    let left_pad = padding / 2;
    let right_pad = padding - left_pad;
    format!(
        "|{}{}{}",
        " ".repeat(left_pad as usize),
        key,
        " ".repeat(right_pad as usize)
    )
}

pub fn render_all_layers(layers: &[String], keymap_dict: &KeymapDictionary) -> String {
    let mut output = String::new();
    let mut layer_index = 0;
    
    for part in layers {
        let mut layer_output = String::new();
        let reg_exp_layer = RegexBuilder::new(r"\w+?,")
            .multi_line(true)
            .build()
            .unwrap();

        layer_output.push_str(&format!("---- LAYER {} start ----\n", layer_index));

        let mut keycode_index = 1;
        let mut row_index = 1;
        for keycode in reg_exp_layer.find_iter(part) {
            let is_end_row = render_keycode_to_string(
                keycode.as_str(),
                keycode_index,
                row_index,
                keymap_dict,
                &mut layer_output,
            );
            if is_end_row {
                layer_output.push_str("|\n");
                print_dashes_to_string(&mut layer_output);
                row_index = row_index + 1;
            }
            keycode_index = keycode_index + 1;
        }
        layer_output.push_str("\n");
        print_dashes_to_string(&mut layer_output);
        layer_output.push_str("---- LAYER end ----\n\n");
        
        output.push_str(&layer_output);
        layer_index = layer_index + 1;
    }
    
    output
}

fn render_keycode_to_string(
    keycode: &str,
    keycode_index: i32,
    row_index: i32,
    keymap_dict: &KeymapDictionary,
    output: &mut String,
) -> bool {
    if keycode_index == 1 {
        print_dashes_to_string(output);
    }
    let is_end_row;
    let is_split_gap;

    let is_thumb_row = IS_SPLIT && row_index > NUMBER_OF_ROWS;

    if is_thumb_row {
        is_end_row = keycode_index % NUMBER_OF_THUMB_KEYS == 0;
        is_split_gap = !is_end_row && keycode_index % (NUMBER_OF_THUMB_KEYS / 2) == 0;
    } else {
        is_end_row = keycode_index % NUMBER_OF_COLUMNS == 0;
        is_split_gap = !is_end_row && IS_SPLIT && keycode_index % (NUMBER_OF_COLUMNS / 2) == 0;
    }

    let mut human_readable = get_key_code_human_readable(&keycode, &keymap_dict);
    human_readable = create_key_gui(&human_readable);
    output.push_str(&human_readable);

    if is_split_gap {
        let repeat_count = KEY_DISPLAY_CHAR_WIDTH;
        output.push_str(&format!("|{}", " ".repeat(repeat_count as usize)));
    }
    is_end_row
}

fn print_dashes_to_string(output: &mut String) {
    let column_width_with_pipes = KEY_DISPLAY_CHAR_WIDTH + 1;
    let num_of_dashes = (column_width_with_pipes * NUMBER_OF_COLUMNS) + 8;
    let line = (0..num_of_dashes)
        .map(|i| {
            if IS_SPLIT
                && i > (num_of_dashes / 2) - (column_width_with_pipes / 2)
                && i < (num_of_dashes / 2) + (column_width_with_pipes / 2)
            {
                ' '
            } else if i % column_width_with_pipes == 0 {
                '+'
            } else {
                '-'
            }
        })
        .collect::<String>();
    output.push_str(&format!("{}\n", line));
}

pub fn open_in_window(text: String) -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(egui::vec2(1000.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "QMK Keymap Visualizer",
        options,
        Box::new(|_cc| Ok(Box::new(KeymapApp::new(text)))),
    )
}

struct KeymapApp {
    lines: Vec<String>,
}

impl KeymapApp {
    fn new(text: String) -> Self {
        Self {
            lines: text.lines().map(String::from).collect(),
        }
    }
}

impl eframe::App for KeymapApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for line in &self.lines {
                    ui.label(egui::RichText::new(line).family(egui::FontFamily::Monospace));
                }
            });
        });
    }
}
