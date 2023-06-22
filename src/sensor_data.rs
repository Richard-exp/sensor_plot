use eframe::egui;
use serialport::SerialPort;
use std::collections::VecDeque;
use std::thread;
use std::time::{Duration, Instant};
use std::io::{self, Write};
pub struct SensorData {
    pub values: VecDeque<egui::plot::PlotPoint>,
    port: Box<dyn SerialPort>,
}

impl SensorData {
    pub fn new() -> Self {
        Self {
            values: VecDeque::new(),
            port: serialport::new("COM3", 9600)
                .timeout(Duration::from_millis(1000))
                .open()
                .expect("Unable to connect COM port"),
        }
    }
    pub fn read_data(mut self) {
        thread::spawn(move || {
            let mut serial_buf: Vec<u8> = vec![0; 100];
            let mut buf_value: f64;
           
            let start_time = Instant::now();
            loop {
                match self.port.read(serial_buf.as_mut_slice()) {
                    Ok(t) => {
                        let time_elapsed = Instant::now() - start_time;                   
                        //io::stdout().write_all(&serial_buf[..t]).unwrap();
                        buf_value = std::str::from_utf8(&serial_buf[..t]).unwrap().parse().unwrap();
                        
                        self.values.push_back(egui::plot::PlotPoint::new(time_elapsed.as_secs_f64(), buf_value));
                    
                       // println!("{buf_value}");
                        //x += 20.0;
                       // Point_vector.push(egui::plot::PlotPoint::new(x, buf_value));
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }  
            }
        }).join().expect("The read_data thread has panicked");
    }
    
    pub fn get_points(self) -> VecDeque<egui::plot::PlotPoint>{
        return self.values;
    }

    pub fn print_points(self) {
        println!("{:?}", self.values);
    }
}