use chrono::Local;
use eframe::egui::{Color32, Window, RichText, TextEdit};

use serde::{Serialize, Deserialize};
use egui::{Context};
use eframe::epi::egui::Layout;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub date_last_edited: String,
    color: [u8; 3],
    pub open: bool
}

impl Note {
    pub fn new(id: i32, text: String, title: String, color: [u8; 3]) -> Self {
        Note {
            id,
            title,
            text,
            date_last_edited: Local::now().to_rfc2822(),
            color,
            open: false
        }
    }

    pub fn get_note_color(&self) -> Color32 {
        Color32::from_rgb(self.color[0], self.color[1], self.color[2])
    }

    // TODO: Fix resizing; add fields
    pub(crate) fn note_window(&mut self, ctx: &Context) {

        // add the popup window for note creation
        let window = Window::new("Edit Note")
            .collapsible(false)
            .resizable(true)
            .open(&mut self.open);
        let m = window
            .show(ctx, |ui| {
                // locking window width
                ui.set_max_width(300.);
                // padding
                ui.add_space(8.);
                // top line title edit widget
                ui.horizontal_top(|ui| {
                    ui.add_space(5.);
                    ui.text_edit_singleline(&mut self.title).on_hover_text(RichText::new("Change Title"));
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
                            });
                        });
                    });

                egui::CentralPanel::default().show_inside(ui, |ui| {
                    let color_edit = ui.color_edit_button_srgb(&mut self.color);
                    let scroll_text = eframe::egui::ScrollArea::vertical()
                        .always_show_scroll(false)
                        .show(ui, |ui| {
                            let text_edit = ui.add_sized(ui.available_size(), TextEdit::multiline(&mut self.text));
                        });

                });
            });
    }
}