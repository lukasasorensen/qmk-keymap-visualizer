use egui::Context;

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
