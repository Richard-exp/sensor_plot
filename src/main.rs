#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui;
use eframe::App;
use egui::plot::PlotBounds;
use serialport::SerialPort;
use std::collections::VecDeque;
use std::io;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let config = SensorPoints::new();
    let port_ref = Arc::clone(&config.port);
    let values_ref = Arc::clone(&config.values);

    let read_values = thread::spawn(move || {
        let mut port_buf: Vec<u8> = vec![0; 3];
        let mut buf_value: f64;
        let start_time = Instant::now();

        loop {
            match port_ref.lock().unwrap().read(port_buf.as_mut_slice()) {
                Ok(t) => {
                    println!("{:?}", port_buf);
                    let time_elapsed = Instant::now() - start_time;
                    buf_value = std::str::from_utf8(&port_buf[..t])
                        .unwrap()
                        .parse()
                        .unwrap();

                    values_ref
                        .lock()
                        .unwrap()
                        .push_back(egui::plot::PlotPoint::new(
                            time_elapsed.as_secs_f64(),
                            buf_value,
                        ));
                    println!("{buf_value}");
                }
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e),
            }
        }
    });

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let gui_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(100.0, 100.0)),
        always_on_top: false,
        ..Default::default()
    };
    eframe::run_native("My egui App", gui_options, Box::new(|_| Box::new(config)))
        .expect("error in GUI");

    read_values
        .join()
        .expect("Error in handling read_port thread");
}

pub struct SensorPoints {
    pub values: Arc<Mutex<VecDeque<egui::plot::PlotPoint>>>,
    pub port: Arc<Mutex<Box<dyn SerialPort>>>,
}

impl SensorPoints {
    fn new() -> Self {
        Self {
            values: Arc::new(Mutex::new(VecDeque::new())),
            port: Arc::new(Mutex::new(
                serialport::new("/dev/ttyACM0", 9600)
                    .timeout(Duration::from_millis(1000))
                    .open()
                    .expect("Unable to connect to port"),
            )),
        }
    }

    fn new_plot_bounds(points: &VecDeque<egui::plot::PlotPoint>) -> PlotBounds {
        let max_point_y = points
            .range((points.len() - 100) as usize..)
            .map(|pp| pp.y as i64)
            .max()
            .expect("Error in  max_point_y") as f64;
        let min_point_y = points
            .range((points.len() - 100) as usize..)
            .map(|pp| pp.y as i64)
            .min()
            .expect("Error in  min_point_y") as f64;
        egui::plot::PlotBounds::from_min_max(
            [points.len() as f64 - 100.0, min_point_y - 5.0],
            [points.len() as f64 + 5.0, max_point_y + 5.0],
        )
    }
}

impl App for SensorPoints {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let points = self.values.lock().unwrap().clone();
        let mut plot = egui::plot::Plot::new("measurements");
        let line = egui::plot::Line::new(egui::plot::PlotPoints::Owned(Vec::from_iter(
            points.iter().copied(),
        )));

        if points.len() <= 100 {
            plot = plot.reset();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            plot.show(ui, |plot_ui| {
                if points.len() > 100 {
                    let my_bounds = Self::new_plot_bounds(&points);
                    plot_ui.set_plot_bounds(my_bounds);
                }
                plot_ui.line(line);
            });

            ui.label("Graph of sensor values");
            if ui.button("Exit").clicked() {
                _frame.close();
            };
        });
        ctx.request_repaint();
    }
}
