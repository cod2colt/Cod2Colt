// my rust debug egui
// use custom fonts to support Chinese
mod customfonts;

use eframe::egui;
use egui::{Button, RichText};
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::{Duration, Instant};

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
    rx: Receiver<()>,
    tx: Sender<()>,
    data_1: String,
    data_2: String,
    output_buffer: String,
    run: bool,
    counter: f64,
}

// impl methods fro my app
impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // set custom font
        customfonts::setup_custom_fonts(&cc.egui_ctx);

        // thread
        let (tx, rx) = mpsc::channel();
        let tx_thread = tx.clone();
        let ctx = cc.egui_ctx.clone();

        // timer thread, trigger per 0.1s
        std::thread::spawn(move || {
            // a fixed time interval to call the thread
            let interval = Duration::from_secs_f64(0.1);
            // get instant now
            let mut next_tick = Instant::now();
            // loop
            loop {
                // get the next thread wake up time
                next_tick += interval;
                // trigger event
                let _ = tx_thread.send(());
                // repaint
                ctx.request_repaint();
                // check the remain time
                let now = Instant::now();
                if next_tick > now {
                    // normal process when we have enough time
                    std::thread::sleep(next_tick - now);
                } else {
                    // behind, reset the time, avoid dead lock
                    if now - next_tick > Duration::from_secs(1) {
                        next_tick = now;
                    }
                }
            }
        });

        // default data
        Self {
            rx,
            tx,
            data_1: "Hello".to_owned(),
            data_2: "World".to_owned(),
            output_buffer: "Hello Rust World".to_owned(),
            run: false,
            counter: 0.0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // run function
        if self.run {
            self.run = false;
            // my rust
            self.output_buffer.clear();
            self.output_buffer = my_rust(&self.data_1, &self.data_2);
        }
        // timer trigger per 0.1 seconds
        if let Ok(_) = self.rx.try_recv() {
            self.counter += 0.1;
        }

        // my egui layout
        egui::CentralPanel::default().show(ctx, |ui| {
            // add label
            ui.vertical_centered(|ui| {
                ui.heading("My Rust egui");
            });
            ui.add_space(20.0);
            // show counter
            let str_output = format!("Counter: {:.1}", self.counter / 10.0);
            ui.label(str_output);

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
