use chrono::{DateTime, Local};
use eframe::egui::Color32;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use std::ops::Not;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Note {
    pub id: i32,
    pub title: String,
    text: String,
    date_last_edited: String,
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
}

// create the app data dir and file
pub fn data_path() -> PathBuf {
    let mut path = dirs_2::home_dir().unwrap();
    path.push(".snow-treading");
    std::fs::create_dir_all(&path).expect("[Snow]: Could not create app dir!");
    path.push("data.json");
    path
}

pub fn load_notes() -> HashMap<i32, Note> {
    let file = File::open(data_path()).expect("Could not read app data!");
    let reader = BufReader::new(file);
    let data: HashMap<i32, Note> = serde_json::from_reader(reader).unwrap();
    println!("{:#?}", data);
    data

}

