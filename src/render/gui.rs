use egui::Context;
use std::collections::HashMap;

pub type KeymapDictionary = HashMap<String, String>;

struct KeymapApp {
    keymap: Vec<Vec<Vec<String>>>,
}

impl KeymapApp {
    fn new(keymap: Vec<Vec<Vec<String>>>) -> Self {
        Self {
            keymap: keymap
        }
    }
}

impl eframe::App for KeymapApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for layers in &self.keymap {
                    for layer in layers {
                        for keycode in layer {
                            ui.label(egui::RichText::new(keycode).family(egui::FontFamily::Monospace));
                        }
                    }
                }
            });
        });
    }
}

pub fn open_keymap_window(keymap: Vec<Vec<Vec<String>>>, keymap_dict: &KeymapDictionary) -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(egui::vec2(1000.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "QMK Keymap Visualizer",
        options,
        Box::new(|_cc| Ok(Box::new(KeymapApp::new(keymap)))),
    )
}
