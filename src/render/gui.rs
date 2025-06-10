use crate::utils::keycode_util;
use egui::Color32;
use egui::Context;
use egui::Stroke;
use std::collections::HashMap;

pub type KeymapDictionary = HashMap<String, String>;

struct KeymapApp {
    keymap: Vec<Vec<Vec<String>>>,
    keymap_dict: KeymapDictionary,
}

impl KeymapApp {
    fn new(keymap: Vec<Vec<Vec<String>>>, keymap_dict: KeymapDictionary) -> Self {
        Self {
            keymap: keymap,
            keymap_dict: keymap_dict,
        }
    }
}

impl eframe::App for KeymapApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut style = (*ctx.style()).clone();

            // Modify the visuals for an active button
            style.visuals.widgets.active.bg_fill = Color32::from_rgb(150, 0, 0);
            style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(200, 200, 200);
            style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(180, 180, 0);
            style.visuals.widgets.active.fg_stroke = Stroke::new(10.0, Color32::from_rgb(255, 255, 255));

            ctx.set_style(style);
            egui::ScrollArea::vertical().show(ui, |ui| {
                for layers in &self.keymap {
                    for row in layers {
                        ui.columns(row.len() as usize, |columns| {
                            for (keycode_index, keycode) in row.iter().enumerate() {
                                let human_readable = keycode_util::get_key_code_human_readable(
                                    keycode,
                                    &self.keymap_dict,
                                );

                                let formatted = format!("\n  {}  \n", human_readable);
                                let rich_text = egui::RichText::new(formatted)
                                    .family(egui::FontFamily::Monospace)
                                    .size(16.0);

                                let button = columns[keycode_index].button(rich_text);
                                if button.clicked() {
                                    println!("Clicked: {}", keycode);
                                }   
                            }
                        });
                    }
                }
            });
        });
    }
}

pub fn open_keymap_window(
    keymap: Vec<Vec<Vec<String>>>,
    keymap_dict: &KeymapDictionary,
) -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(egui::vec2(1000.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "QMK Keymap Visualizer",
        options,
        Box::new(|_cc| Ok(Box::new(KeymapApp::new(keymap, keymap_dict.clone())))),
    )
}
