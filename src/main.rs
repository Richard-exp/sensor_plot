#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod sensor_data_1;
// use sensor_data_1::SensorData;

use eframe::egui;
// use egui::Vec2;
// use std::io::{self, Write};

use std::collections::VecDeque;
// use std::sync::{Arc, Mutex};
// use std::thread;

use serialport::SerialPort;
use std::thread;
use std::time::{Duration, Instant};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

fn main() {
    let app = MyApp::new_port();
    // let app_2 = MyApp::new_app();
    // let mut app = MyApp::new_app();
    let port_ref_2 = Arc::new(Mutex::new(&app.port));
    let values_ref_2 = Arc::new(Mutex::new(&app.values));
    let port_ref_2_copy = Arc::clone(&port_ref_2);
    let values_ref_2_copy = Arc::clone(&values_ref_2);
    // let port_ref = Arc::clone(&app.port);
    // let values_ref = Arc::clone(&app.values);
    let port_reading = thread::spawn(move || {
        let mut serial_buf: Vec<u8> = vec![0; 100];
        let mut buf_value: f64;
        let start_time = Instant::now();
        loop {
            match port_ref_2_copy.lock().unwrap().read(serial_buf.as_mut_slice()) {
                Ok(t) => {
                    let time_elapsed = Instant::now() - start_time;                   
                    //io::stdout().write_all(&serial_buf[..t]).unwrap();
                    buf_value = std::str::from_utf8(&serial_buf[..t]).unwrap().parse().unwrap();
                    
                    values_ref_2_copy.lock().unwrap().push_back(egui::plot::PlotPoint::new(time_elapsed.as_secs_f64(), buf_value));
                
                    //println!("{buf_value}");
                    //x += 20.0;
                // Point_vector.push(egui::plot::PlotPoint::new(x, buf_value));
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
    port_reading.join().expect("error in handling thread_1");
}

pub struct MyApp {
    pub values: VecDeque<egui::plot::PlotPoint>,
    pub port: Box<dyn SerialPort>,
}

impl MyApp {
    fn new_port() -> Self {
        Self {
            values: VecDeque::new(),
            port: serialport::new("COM3", 9600)
            .timeout(Duration::from_millis(1000))
            .open()
            .expect("Unable to connect COM port"),
        }   
    } 

    // fn read_data (&mut self) {
    //             let mut serial_buf: Vec<u8> = vec![0; 100];
    //             let mut buf_value: f64;
            
    //             let start_time = Instant::now();
    //             loop {
    //                 match self.port.lock().unwrap().read(serial_buf.as_mut_slice()) {
    //                     Ok(t) => {
    //                         let time_elapsed = Instant::now() - start_time;                   
    //                         //io::stdout().write_all(&serial_buf[..t]).unwrap();
    //                         buf_value = std::str::from_utf8(&serial_buf[..t]).unwrap().parse().unwrap();
                            
    //                         self.values.lock().unwrap().push_back(egui::plot::PlotPoint::new(time_elapsed.as_secs_f64(), buf_value));
                        
    //                     // println!("{buf_value}");
    //                         //x += 20.0;
    //                     // Point_vector.push(egui::plot::PlotPoint::new(x, buf_value));
    //                     }
    //                     Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
    //                     Err(e) => eprintln!("{:?}", e),
    //                 } 
    //             }
    // }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Exit").clicked() {
                _frame.close();
            };
            
            let mut plot = egui::plot::Plot::new("measurements").auto_bounds_y();

            plot.show(ui, |plot_ui| {
                //let points = self.data.get_points();
                plot_ui.line(
                    egui::plot::Line::new(egui::plot::PlotPoints::Owned(Vec::from_iter(
                        self.values.iter().copied(),
                    ))), //self.measurements.lock().unwrap().plot_values(),
                );
            });
        });
    }
}