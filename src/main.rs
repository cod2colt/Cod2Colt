// my rust egui

use eframe::egui;

fn main() -> eframe::Result {
    // set initialize windows size
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    // create my app
    eframe::run_native(
        "My Rust egui app",
        options,
        Box::new(|cc| Ok(Box::<MyApp>::default())),
    )
}

// create My app structure
struct MyApp {
    name: String,
}

// impl methods fro my app
impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Hello Rust egui".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // my egui layout
        egui::CentralPanel::default().show(ctx, |ui| {
            // add label
            ui.heading("My Rust egui");
            // a input
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
                ui.heading(&self.name);
            });
        });
    }
}
