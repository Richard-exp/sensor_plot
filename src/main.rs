#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui;
use eframe::egui::Vec2;
use eframe::App;
use serialport::SerialPort;
use std::collections::VecDeque;
use std::io;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let app = SensorPoints::new_port();
    let port_ref = Arc::clone(&app.port);
    let values_ref = Arc::clone(&app.values);
    let read_port = thread::spawn(move || {
        let mut serial_buf: Vec<u8> = vec![0; 100];
        let mut buf_value: f64;

        let start_time = Instant::now();
        loop {
            match port_ref.lock().unwrap().read(serial_buf.as_mut_slice()) {
                Ok(_t) => {
                    println!("{:?}", serial_buf);
                    let time_elapsed = Instant::now() - start_time;
                    buf_value = std::str::from_utf8(&serial_buf[..3])
                        .unwrap()
                        .parse()
                        .unwrap();
                    // if (values_ref.lock().unwrap().len() > 100) {
                    //     values_ref.lock().unwrap().pop_front();
                    // }
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
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(100.0, 100.0)),
        always_on_top: false,
        ..Default::default()
    };
    eframe::run_native("My egui App", options, Box::new(|_| Box::new(app))).expect("error in UI");
    read_port
        .join()
        .expect("Error in handling read_port thread");
}

pub struct SensorPoints {
    pub values: Arc<Mutex<VecDeque<egui::plot::PlotPoint>>>,
    pub port: Arc<Mutex<Box<dyn SerialPort>>>,
}

impl SensorPoints {
    fn new_port() -> Self {
        Self {
            values: Arc::new(Mutex::new(VecDeque::new())),
            port: Arc::new(Mutex::new(
                serialport::new("COM3", 9600)
                    .timeout(Duration::from_millis(1000))
                    .open()
                    .expect("Unable to connect to COM port"),
            )),
        }
    }
}

impl App for SensorPoints {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Graph of sensor values");
            if ui.button("Exit").clicked() {
                _frame.close();
            };

            let points = self.values.lock().unwrap().clone();
            let plot = egui::plot::Plot::new("measurements");
            let line = egui::plot::Line::new(egui::plot::PlotPoints::Owned(Vec::from_iter(
                points.iter().copied(),
            )));

            // for i in points.iter() {
            //     plot = plot.include_x(i.x);
            //     plot = plot.include_y(i.y);
            // }

            // if points.len() <= 10 {
            //     plot = plot.reset();
            // }

            // let last_point_x = points.get(points.len()).unwrap().x;
            // let last_point_y = points.get(points.len()).unwrap().y;

            plot.show(ui, |plot_ui| {
                if points.len() > 10 {
                    if let Some(last_point) = { points.get(points.len()) } {
                        let last_point_x = last_point.x;
                        let last_point_y = last_point.y;
                        let my_bounds = egui::plot::PlotBounds::from_min_max(
                            [last_point_x - 100.0, last_point_y],
                            [last_point_x, last_point_y],
                        );
                        plot_ui.set_plot_bounds(my_bounds);
                    }
                }
                plot_ui.line(line);
            });

            // .transform.position_from_point({
            //     match points.get(points.len()) {
            //         Some(t) => t,
            //         None => &zero_point,
            //     }
            // });
        });
    }
}
