use eframe::epi;

struct App {}

impl epi::App for App {
    fn name(&self) -> &str {
        "Monitoring App"
    }
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, add_contents)
    }
}

fn main_1() {
    let app = App {};
    let native_options = Default::default(); 
    eframe::run_native(Box::new(app), native_options);
  println!("Hello, world!");
}