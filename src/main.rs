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
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::Vec2::new(1920.0 / 2.0, 1028.0 / 2.0)),
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
    function: String,
    data: String,
    output_buffer: String,
    counter: f64,
    hello: bool,
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
                        let function = value.get(0).map(|s| s.as_str()).unwrap();
                        let data = value.get(1).map(|s| s.as_str()).unwrap();
                        let data_output = my_rust(function, data);
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
            function: "Rust".to_owned(),
            data: "Rustaceans".to_owned(),
            output_buffer: "Hello Rust World".to_owned(),
            counter: 0.0,
            hello: true,
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
                    self.counter %= 10000.0;
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

            // input
            ui.horizontal(|ui| {
                // show counter
                ui.label(egui::RichText::new("Counter: ").size(14.0));
                let str_output = format!("{:06.1}s", self.counter);
                ui.label(egui::RichText::new(str_output).monospace().size(14.0));
                ui.add_space(5.0);

                // input data 1
                let function_label = ui.label("Function: ");
                egui::Frame::NONE
                    .stroke(egui::Stroke::new(1.0, egui::Color32::LIGHT_GRAY))
                    .show(ui, |ui| {
                        ui.text_edit_singleline(&mut self.function)
                            .labelled_by(function_label.id)
                    });
                ui.add_space(5.0);

                // input data 2
                let data_label = ui.label("Data: ");
                egui::Frame::NONE
                    .stroke(egui::Stroke::new(1.0, egui::Color32::LIGHT_GRAY))
                    .show(ui, |ui| {
                        ui.text_edit_singleline(&mut self.data)
                            .labelled_by(data_label.id)
                    });

                ui.add_space(5.0);

                // Run button
                let mut run_clicked = false; // reset per frame 
                // show the hello frame
                if self.hello {
                    self.function = "Rust".to_string();
                    self.data = "Rustaceans".to_string();
                    self.hello = false;
                    run_clicked = true;
                }
                // get enter key
                if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    run_clicked = true;
                }
                let run_button = Button::new(RichText::new("Run").size(16.0));
                if ui.add(run_button).clicked() {
                    run_clicked = true;
                };
                if run_clicked {
                    // get data input
                    self.function = self.function.trim().to_string();
                    self.data = self.data.trim().to_string();

                    // thread: trig by button hit
                    let data_send = Msg::Data {
                        from: ThreadId::Ui,
                        value: vec![self.function.clone(), self.data.clone()],
                    };
                    self.tx_work.send(data_send).unwrap();
                };
                // exit button
                let exit_button = Button::new(RichText::new("Exit").size(16.0));
                if ui.add(exit_button).clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                };
            });
            // add test code run buttons
            ui.horizontal(|ui| {
                for my_fun in myrust::MY_TEST_FUN.iter() {
                    let run_button = Button::new(RichText::new(my_fun.name).size(16.0));
                    if ui.add(run_button).clicked() {
                        // run Data
                        // get data input
                        self.function = my_fun.name.to_string();
                        self.data = self.data.trim().to_string();

                        // thread: trig by button hit
                        let data_send = Msg::Data {
                            from: ThreadId::Ui,
                            value: vec![self.function.clone(), self.data.clone()],
                        };
                        self.tx_work.send(data_send).unwrap();
                    };
                }
            });
            // label to print the data
            ui.label("Output 👉");
            ui.add_space(2.0);
            egui::ScrollArea::vertical()
                .max_height(1024.0)
                .show(ui, |ui| {
                    ui.label(egui::RichText::new(&self.output_buffer).monospace());
                });
        });
    }
}
