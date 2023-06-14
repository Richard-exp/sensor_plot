#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use std::io::{self, Write};
use std::time::{Instant, Duration};

fn main() -> Result<(), eframe::Error> {

    //----------------------------------------------------------------------------------------------------------- 
let mut port = serialport::new("COM3", 9600)
.timeout(Duration::from_millis(1000))
.open();
let mut Point_vector: Vec<egui::plot::PlotPoint> = Vec::new();

match port {
    Ok(mut port) => {
        let mut serial_buf: Vec<u8> = vec![0; 100];
        let mut buf_value: f64;
        let mut x: f64 = -100.0;
        for i in 0..10 {
            match port.read(serial_buf.as_mut_slice()) {
                Ok(t) => { 
                //io::stdout().write_all(&serial_buf[..t]).unwrap(); 
                if i > 0 {
                buf_value = std::str::from_utf8(&serial_buf[..t]).unwrap().parse().unwrap();
                println!("{buf_value}");
                x+=20.0;
                Point_vector.push(egui::plot::PlotPoint::new(x, buf_value));}
                }
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e),
            }
        }
    }
    Err(e) => {
        ::std::process::exit(1);
    }
}
//-----------------------------------------------------------------------------------------------------------
    
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(100.0, 100.0)), 
        always_on_top : false,
        
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    name: String,
    age: u32,
    is_open: bool,
    measurements: i64,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            is_open: true,
            measurements: 0,
        }
    }
}

impl eframe::App for MyApp {
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

    egui::CentralPanel::default().show(ctx, |ui| {
    if  ui.button("Exit").clicked(){
    _frame.close();
  };
  
  let mut plot = egui::plot::Plot::new("measurements");

  plot.show(ui, |plot_ui| {
    plot_ui.line(egui::plot::Line::new(
        egui::plot::PlotPoints::Owned(Vec::from_iter(Point_vector.iter().copied())))
        //self.measurements.lock().unwrap().plot_values(),
    );
});
        });
    }
}
