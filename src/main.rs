#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui;
use std::io;
use std::collections::VecDeque;
use serialport::SerialPort;
use std::thread;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};

fn main() {
    let app = MyApp::new_port();
    let port_ref = Arc::clone(&app.port);
    let values_ref = Arc::clone(&app.values);
    let read_port = thread::spawn(move || {
        let mut serial_buf: Vec<u8> = vec![0; 100];
        let mut buf_value: f64;
    
        let start_time = Instant::now();
        loop {
            match port_ref.lock().unwrap().read(serial_buf.as_mut_slice()) {
                Ok(t) => {
                    let time_elapsed = Instant::now() - start_time;                   
                    buf_value = std::str::from_utf8(&serial_buf[..t]).unwrap().parse().unwrap();
                    
                    values_ref.lock().unwrap().push_back(egui::plot::PlotPoint::new(time_elapsed.as_secs_f64(), buf_value));
                    //println!("{buf_value}");
                }
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e),
            } 
        }
});
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(100.0, 100.0)),
        always_on_top: false,

        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_| Box::new(app)),
    ).expect("error in UI");
    read_port.join().expect("Error in handling read_port thread");
}

pub struct MyApp {
    pub values: Arc<Mutex<VecDeque<egui::plot::PlotPoint>>>,
    pub port: Arc<Mutex<Box<dyn SerialPort>>>,
}

impl MyApp {
    fn new_port() -> Self {
        Self {
            values: Arc::new(Mutex::new(VecDeque::new())),
            port: Arc::new(Mutex::new(serialport::new("COM3", 9600)
            .timeout(Duration::from_millis(1000))
            .open()
            .expect("Unable to connect to COM port"))),
        }   
    } 
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Exit").clicked() {
                _frame.close();
            };
            
            let plot = egui::plot::Plot::new("measurements").auto_bounds_y();

            plot.show(ui, |plot_ui| {
                plot_ui.line(
                    egui::plot::Line::new(egui::plot::PlotPoints::Owned(Vec::from_iter(
                        self.values.lock().unwrap().iter().copied(),
                    ))), 
                );
            });
        });
    }
}