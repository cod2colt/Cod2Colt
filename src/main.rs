// my rust debug egui
// use custom fonts to support Chinese
mod customfonts;

use eframe::egui;
use egui::{Button, RichText};

// my rust
mod myrust;
use myrust::my_rust;

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
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

// create My app structure
struct MyApp {
    data_1: String,
    data_2: String,
    output_buffer: String,
    run: bool,
}

// impl methods fro my app
impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // set custom font
        customfonts::setup_custom_fonts(&cc.egui_ctx);
        // default data
        Self {
            data_1: "Hello".to_owned(),
            data_2: "World".to_owned(),
            output_buffer: "Hello Rust World".to_owned(),
            run: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.run {
            self.run = false;
            // my rust
            self.output_buffer.clear();
            self.output_buffer = my_rust(&self.data_1, &self.data_2);
        }

        // my egui layout
        egui::CentralPanel::default().show(ctx, |ui| {
            // add label
            ui.heading("My Rust egui");
            ui.add_space(20.0);

            // input
            ui.horizontal(|ui| {
                // input data 1
                let data_1_label = ui.label("Data 1: ");
                ui.text_edit_singleline(&mut self.data_1)
                    .labelled_by(data_1_label.id);
                // input data 2
                let data_2_label = ui.label("Data 2: ");
                ui.text_edit_singleline(&mut self.data_2)
                    .labelled_by(data_2_label.id);
            });
            ui.add_space(20.0);
            // Run button
            let run_button = Button::new(RichText::new("Run").size(18.0));
            if ui.add(run_button).clicked() {
                self.run = true;
                self.data_1 = self.data_1.trim().to_string();
                self.data_2 = self.data_2.trim().to_string();
            };
            // label to print the data
            ui.add_space(20.0);
            ui.label("Output:");
            ui.add_space(2.0);
            ui.label(format!("{}", &self.output_buffer));
        });
    }
}
