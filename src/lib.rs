mod cloud;

use chrono::{Local};
use eframe::egui::{Color32, Context, Window, Vec2, Button};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufReader, Error, BufWriter};
use serde::de::DeserializeOwned;

#[macro_use]
extern crate log;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub date_last_edited: String,
    color: [u8; 4]
}

impl Note {
    pub fn new(id: i32, text: String, title: String, color: [u8; 4]) -> Self {
        Note {
            id,
            title,
            text,
            date_last_edited: Local::now().to_rfc2822(),
            color,
        }
    }

    pub fn get_note_color(&self) -> Color32 {
        Color32::from_rgb(self.color[0], self.color[1], self.color[2])
    }
}

// create the app data dir and file
pub fn data_path(file: &str) -> PathBuf {
    let mut path = dirs_2::home_dir().unwrap();
    path.push(".snow-treading");
    std::fs::create_dir_all(&path).expect("[Snow]: Could not create app dir!");
    path.push(format!("{}.json", file));
    path
}

pub fn load_file<T: DeserializeOwned>(file: &str) -> Vec<T> {
    let file = File::open(data_path(file)).expect("Could not read app data!");
    let reader = BufReader::new(file);
    let data: Vec<T> = serde_json::from_reader(reader).unwrap();
    data

}

pub fn save_file<T: Serialize>(file_name: &str, data: T) -> Result<(), Error> {
    let file = File::create(data_path(file_name)).expect("Unable to create/read file!");
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &data);
    info!("saved '{}' to '{}'!", file_name, "Dummy for where its stored - e.g Local, Cloud etc");
    Ok(())
}

pub fn config_window(ctx: &Context, mut open: &mut bool) {

    let window = Window::new("configuration")
        .title_bar(false)
        .collapsible(false);
    let response = window.show(ctx, |ui| {

        ui.set_max_width(80.);

        egui::TopBottomPanel::top("config_top").show_inside(ui, |ui| {
            ui.label("configuration");
            let close_btn = ui.add(Button::new("X"));
            if close_btn.clicked() {

            }
        });
        egui::TopBottomPanel::bottom("bottom").show_inside(ui, |ui| {

        });
        egui::CentralPanel::default()
            .show_inside(ui, |ui| {
                ui.set_max_height(50.);
                ui.set_max_width(80.);
                ui.horizontal(|ui| {
                    ui.add_space(10.);
                    let drive_btn = ui.add(Button::new("Google Drive"));
                });

            });


    });
}


