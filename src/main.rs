// my rust debug egui
// system module
use eframe::egui;
use egui::{Button, RichText};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

// my module
mod customfonts;
use customfonts::setup_custom_fonts;
mod myrust;
use myrust::my_rust;

// thread
#[derive(Debug, Clone, Copy)]
enum ThreadId {
    Timer,
    Worker,
    Ui,
}

#[derive(Debug)]
enum Msg {
    Tick { from: ThreadId },
    Data { from: ThreadId, value: Vec<String> },
}

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
    rx: Receiver<Msg>,
    tx_work: Sender<Msg>,
    data_1: String,
    data_2: String,
    output_buffer: String,
    counter: f64,
}

// impl methods fro my app
impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // set custom font
        setup_custom_fonts(&cc.egui_ctx);

        // timer thread, trigger per 0.1s
        let (tx, rx) = mpsc::channel();
        let tx_timer = tx.clone();
        let ctx = cc.egui_ctx.clone();

        thread::spawn(move || {
            // a fixed time interval to call the thread
            let interval = Duration::from_secs_f64(0.1);
            // get instant now
            let mut next_tick = Instant::now();
            // loop
            loop {
                // get the next thread wake up time
                next_tick += interval;
                // trigger event
                let _ = tx_timer
                    .send(Msg::Tick {
                        from: ThreadId::Timer,
                    })
                    .unwrap();
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

        // work thread
        let (tx_work, rx_work) = mpsc::channel();
        let tx_working = tx.clone();
        let ctx = cc.egui_ctx.clone();

        thread::spawn(move || {
            while let Ok(msg) = rx_work.recv() {
                match msg {
                    Msg::Data {
                        from: ThreadId::Ui,
                        value,
                    } => {
                        // get data from ui and prepare data for my_rust
                        let data_0 = value.get(0).map(|s| s.as_str()).unwrap();
                        let data_1 = value.get(1).map(|s| s.as_str()).unwrap();
                        let data_output = my_rust(data_0, data_1);
                        // send data back to ui
                        let data_send = Msg::Data {
                            from: ThreadId::Worker,
                            value: vec![data_output],
                        };
                        tx_working.send(data_send).unwrap();
                        // repaint when job finish
                        ctx.request_repaint();
                    }
                    _ => {}
                }
            }
        });

        // default data
        Self {
            rx,
            tx_work,
            data_1: "".to_owned(),
            data_2: "".to_owned(),
            output_buffer: "Hello Rust World".to_owned(),
            counter: 0.0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle threads
        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                // Timer trigger per 0.1 seconds
                Msg::Tick {
                    from: ThreadId::Timer,
                } => {
                    self.counter += 0.1;
                }
                // Work thread process
                Msg::Data {
                    from: ThreadId::Worker,
                    value,
                } => {
                    self.output_buffer.clear();
                    self.output_buffer = value.join("\n");
                }
                _ => {}
            }
        }

        // my egui layout
        egui::CentralPanel::default().show(ctx, |ui| {
            // add label
            ui.vertical_centered(|ui| {
                ui.heading("My Rust egui");
            });
            ui.add_space(20.0);
            // show counter
            let str_output = format!("Counter: {:.1}s", self.counter);
            ui.label(egui::RichText::new(str_output).monospace().size(14.0));

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
            let mut run_clicked = false; // reset per frame 
            // get enter key
            if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                run_clicked = true;
            }
            let run_button = Button::new(RichText::new("Run").size(18.0));
            if ui.add(run_button).clicked() {
                run_clicked = true;
            };
            if run_clicked {
                // get data input
                self.data_1 = self.data_1.trim().to_string();
                self.data_2 = self.data_2.trim().to_string();

                // thread: trig by button hit
                let data_send = Msg::Data {
                    from: ThreadId::Ui,
                    value: vec![self.data_1.clone(), self.data_2.clone()],
                };
                self.tx_work.send(data_send).unwrap();
            };
            // add test code run buttons
            ui.horizontal(|ui| {
                let run_button = Button::new(RichText::new("Data").size(18.0));
                if ui.add(run_button).clicked() {
                    // run Data
                    // get data input
                    self.data_1 = "Data".to_string();
                    self.data_2 = self.data_2.trim().to_string();

                    // thread: trig by button hit
                    let data_send = Msg::Data {
                        from: ThreadId::Ui,
                        value: vec![self.data_1.clone(), self.data_2.clone()],
                    };
                    self.tx_work.send(data_send).unwrap();
                };
            });
            // label to print the data
            ui.add_space(20.0);
            ui.label("Output 👉");
            ui.add_space(2.0);
            egui::ScrollArea::vertical()
                .max_height(1024.0)
                .show(ui, |ui| {
                    ui.label(format!("{}", &self.output_buffer));
                });
        });
    }
}
