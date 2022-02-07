use chrono::{Local};
use eframe::egui::Color32;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;


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
pub fn data_path() -> PathBuf {
    let mut path = dirs_2::home_dir().unwrap();
    path.push(".snow-treading");
    std::fs::create_dir_all(&path).expect("[Snow]: Could not create app dir!");
    path.push("data.json");
    path
}

pub fn load_notes() -> Vec<Note> {
    let file = File::open(data_path()).expect("Could not read app data!");
    let reader = BufReader::new(file);
    let data: Vec<Note> = serde_json::from_reader(reader).unwrap();
    println!("{:#?}", data);
    data

}

