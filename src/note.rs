use chrono::Local;
use eframe::egui::{Color32, Window, RichText, TextEdit, Button, TextStyle, Id};

use serde::{Serialize, Deserialize};
use egui::{Context, Vec2};
use eframe::epi::egui::Layout;
use snow_treading::save_file;
use std::collections::HashMap;
use std::ops::Index;
use egui::text_edit::CursorRange;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NoteWarp {
    pub(crate) notes: Vec<Note>,
    pub(crate) confirmation_window: (bool, String),
    pub(crate) bool: bool,
    pub(crate) closing_window: bool,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub date_last_edited: String,
    color: [u8; 3],
}

impl Note {
    pub fn new(id: i32, text: String, title: String, color: [u8; 3]) -> Self {
        let title_len = title.len();
        let text_len = text.len();
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

impl NoteWarp {

    // TODO: Character count for title and text
    pub(crate) fn note_window(&mut self, ctx: &Context, index: usize) {

        let title_len = self.notes[index].title.len() as f32;

        // add the popup window for note creation
        let window = Window::new("Edit Note")
            .id(Id::new("note_edit_window"))
            .collapsible(false)
            .resizable(true)
            .title_bar(false);
        let m = window
            .show(ctx, |ui| {
                // locking window width
                ui.set_max_width(300.);
                // padding
                //ui.add_space(8.);
                //TODO: Top Field, Asking to save before exit
                egui::menu::bar(ui, |ui| {
                        ui.with_layout(Layout::right_to_left(), |ui| {
                            let mut close_btn = ui.add(Button::new(RichText::new("⛔")));
                            if close_btn.clicked() {
                                self.bool = false;
                            }
                            });
                        });

                ui.separator();
                ui.add_space(5.);
                // top line title edit widget TODO: Fix max characters!
                ui.horizontal_top(|ui| {
                    ui.add_space(5.);
                    let mut title_field = TextEdit::singleline(&mut self.notes[index].title)
                        .id(Id::new("text_file"))
                        .margin(Vec2::new(if title_len < 40. {120. - (title_len * 2.5)} else {8.}, 5.))
                        .font(TextStyle::Heading)
                        .show(ui);

                    // ui.text_edit_singleline(&mut self.notes[index].title).on_hover_text(RichText::new("Change Title"));
                });
                ui.add_space(10.);

                // bottom panel for functional buttons
                egui::TopBottomPanel::bottom("bottom_note")
                    .resizable(false)
                    .show_inside(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.set_height(30.);
                            ui.with_layout(Layout::right_to_left(), |ui| {
                                // saving button
                                let save_note_btn = ui.button(RichText::new("Save").strong().heading());
                                if save_note_btn.clicked() {
                                    save_file("data", &self.notes);
                                    self.confirmation_window = (true, "saved!".to_string());
                                    let mut count = 0;
                                    while count <= 200 {
                                        count += 1;
                                    }
                                    self.confirmation_window = (false, "".to_string())
                                }
                            });
                        });
                    });

                // central panel containing text-edit, color picker etc...
                egui::CentralPanel::default().show_inside(ui, |ui| {

                    // TODO: make this prettier
                    ui.horizontal(|ui| {
                        let color_edit = ui.color_edit_button_srgb(&mut self.notes[index].color);
                    });

                    let scroll_text = eframe::egui::ScrollArea::vertical()
                        .always_show_scroll(false)
                        .show(ui, |ui| {
                            let text_edit = ui.add_sized(
                                ui.available_size(),
                                TextEdit::multiline(&mut self.notes[index].text));
                        });

                });
        });
    }
}
