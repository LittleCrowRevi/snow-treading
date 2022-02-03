use app::SnowApp;
use egui::Vec2;
mod app;

fn main() {
    println!("Hello, world!");
    let app = SnowApp::default();
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(850., 560.));
    eframe::run_native(Box::new(app), native_options)

}
