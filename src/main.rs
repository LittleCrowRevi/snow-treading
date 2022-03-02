
use crate::app::SnowApp;
use eframe::egui::{Vec2, Visuals, Color32};
use reqwest;

mod app;
mod note;
mod config;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    pretty_env_logger::init();

    info!("Hello, world!");
    let app = SnowApp::new();
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(750., 760.));
    native_options.min_window_size = Some(Vec2::new(750., 440.));

    eframe::run_native(Box::new(app), native_options);
}
