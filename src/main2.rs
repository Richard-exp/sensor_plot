#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod sensor_data_1;
use sensor_data_1::SensorData;

use egui::plot::{Line, Plot, PlotPoints};
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    // Our application state:
    let mut app_1 = SensorData::new();
    app_1.read_data();
    let points = app_1.get_points();

    eframe::run_simple_native ("My egui App", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            // let sin: PlotPoints = (0..1000).map(|i| {
            //     let x = i as f64 * 0.01;
            //     [x, x.sin()]
            // }).collect();
            // let line = Line::new(sin);
            // Plot::new("my_plot").view_aspect(2.0).show(ui, |plot_ui| plot_ui.line(line));
            if ui.button("Exit").clicked() {
                _frame.close();
            };
            
            let plot = egui::plot::Plot::new("measurements").auto_bounds_y();
            plot.show(ui, |plot_ui| {
                plot_ui.line(
                    egui::plot::Line::new(egui::plot::PlotPoints::Owned(Vec::from_iter(
                        points.iter().copied(),
                    ))), //self.measurements.lock().unwrap().plot_values(),
                );
            });

        });
    })
}

