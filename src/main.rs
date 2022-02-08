use egui::Vec2;
use crate::app::SnowApp;
mod app;

fn main() {
    println!("Hello, world!");
    let app = SnowApp::new();
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(850., 560.));
    native_options.min_window_size = Some(Vec2::new(750., 440.));
    eframe::run_native(Box::new(app), native_options)

}
