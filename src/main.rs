#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod sensor_data;
use sensor_data::SensorData;

use eframe::egui;
use egui::Vec2;
use std::io::{self, Write};

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() -> Result<(), eframe::Error> {
    //let counter = Arc::new(Mutex::new(0));
    let mut app = MyApp::new();
    app.data.read_data();
    //app.data.print_points(); - How to do this?

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
    )
}

pub struct MyApp {
   data: SensorData,
}

impl MyApp {
    fn new() -> Self {
        Self {
            data: SensorData::new(),
        }   
    }    
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Exit").clicked() {
                _frame.close();
            };
            
            let mut plot = egui::plot::Plot::new("measurements").auto_bounds_y();

            plot.show(ui, |plot_ui| {
                let points = self.data.get_points();
                plot_ui.line(
                    egui::plot::Line::new(egui::plot::PlotPoints::Owned(Vec::from_iter(
                        points.iter().copied(),
                    ))), //self.measurements.lock().unwrap().plot_values(),
                );
            });
        });
    }
}
