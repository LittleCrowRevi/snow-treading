use std::borrow::{Cow, Borrow};

use serde::{Deserialize, Serialize};
use eframe::egui::{Button, Color32, Context, Direction, FontData, FontDefinitions, FontFamily,
                   Label, Layout, RichText, TextStyle, TopBottomPanel, Ui, Visuals, FontId,
                   TextBuffer, Stroke, Vec2, Rgba, Window, Rect};
use eframe::epi::Frame;
use eframe::epi;
use epi::App;
use egui::{ScrollArea};
use eframe::epi::Storage;
use std::time::Duration;
use snow_treading::{load_file, save_file};
use crate::note::{Note, NoteWarp};
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Range;


// simple config struct
#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone)]
pub struct AppConfig {
    pub(crate) dark_mode: bool,
    bookmark_panel: bool
}

impl AppConfig {
    fn new() -> Self {
        AppConfig { dark_mode: true, bookmark_panel: true }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            dark_mode: true,
            bookmark_panel: true
        }
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct SnowApp {
    label: String,
    // an empty label for text inputs?
    pub(crate) empty_label: String,
    config: AppConfig,
    note_warp: NoteWarp,
    config_window: bool,
    note: Option<usize>,
    confirmation_window: (bool, String)
}

impl App for SnowApp {

    // everything here gets rendered all the time
    fn update(&mut self, ctx: &Context, frame: &epi::Frame) {
        // create Visuals object
        // inherits all the default values from dark and light based on theme mode
        // ease of changing default visuals this way
        let mut visuals = Visuals { ..if self.config.dark_mode {Visuals::dark()} else {Visuals::light()} };


        // change bg-color and other stuff depending onf theme mode
        match self.config.dark_mode {
            true => {
                visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(30, 30, 30);
                ctx.set_visuals(visuals);
            }
            false => {
                visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(249, 249, 244);
                visuals.override_text_color = Some(Color32::from_rgb(0, 0, 0));
                ctx.set_visuals(visuals);
            }
        }

        if self.note_warp.bool {
            self.note_warp.note_window(ctx, self.note.unwrap());
        }


        if self.note_warp.confirmation_window.0 {
            Window::new("saved!")
                .collapsible(false)
                .show(ctx, |ui| {});

        }

        // call top panel render
        self.render_top_panel(ctx, frame);

        // side panel containing quick access to recent or bookmarked notes TODO: add timestamps and sort
        self.render_bookmarks_panel(ctx);

        // TODO: EVERYTHING
        self.center_panel_render(ctx);
    }

    fn setup(&mut self, ctx: &Context, _frame: &epi::Frame, _storage: Option<&dyn epi::Storage>) {
        self.configure_fonts(ctx);
    }

    fn save(&mut self, _storage: &mut dyn Storage) {

    }

    fn on_exit(&mut self) {
    }

    fn name(&self) -> &str {
        "Snow Window"
    }

    fn auto_save_interval(&self) -> Duration {
        Duration::new(300, 0)
    }

}

impl SnowApp {

    pub fn new() -> SnowApp{

        let config: AppConfig = confy::load("Snow Window").unwrap_or_default();

        SnowApp {
            label: String::from("Hallo, Snowy World"),
            empty_label: "".to_owned(),
            config,
            note_warp: NoteWarp {
                notes: load_file("data"),
                confirmation_window: (false, "".to_string()),
                bool: false,
                closing_window: false
            },
            config_window: false,
            note: None,
            confirmation_window: (false, "".to_string())
        }
    }

    fn render_top_panel(&mut self, ctx: &Context, frame: &Frame) {
        TopBottomPanel::top("header").show(ctx, |ui| {
            ui.add_space(3.3);

            egui::menu::bar(ui, |ui| {

                //add the snow-symbol to the top bar
                ui.with_layout(Layout::left_to_right(), |ui| {
                    ui.add(Label::new(
                        RichText::new("❄").text_style(TextStyle::Heading).strong(),
                    ));

                    // button for hiding(showing?) the bookmarks side panel
                    let bookmarks_btn = ui.add(Button::new(RichText::new("=").strong().heading()))
                        .on_hover_text(RichText::new(if self.config.bookmark_panel {"Hide Bookmarks"} else {"Show Bookmarks"}));
                    if bookmarks_btn.clicked() {
                        self.config.bookmark_panel = !self.config.bookmark_panel;
                        self.store_confy();
                    }
                });

                // add top bar title string
                ui.with_layout(
                    Layout::centered_and_justified(Direction::RightToLeft),
                    |ui| {
                        ui.add(Label::new(
                            RichText::new("Hello, snowy world!")
                                .color(match self.config.dark_mode {
                                    true => Color32::WHITE,
                                    false => Color32::BLACK,
                                })
                                .text_style(TextStyle::Heading)
                                .font(FontId::new(20., FontFamily::Proportional)),
                        ))
                    },
                );

                // add the closing, menu and theme button
                ui.with_layout(Layout::right_to_left(), |ui| {
                    ui.add_space(4.);
                    let close_btn = ui
                        .add(Button::new(RichText::new("⛔").heading()))
                        .on_hover_text(RichText::new("Exit App"));
                    let config_btn = ui
                        .add(Button::new(
                            RichText::new("⚙️")
                                .heading()))
                        .on_hover_text(RichText::new("Config"));
                    let theme_btn = ui
                        .add(Button::new(
                            RichText::new(if self.config.dark_mode {"Dark"} else {"Light"})
                                .strong()
                                .font(FontId::proportional(17.))))
                        .on_hover_text(RichText::new("Change theme!"));

                    // add logic to the close button
                    if close_btn.clicked() {
                        dbg!("Closing app!");
                        frame.quit();
                    };

                    //add logic to the theme button
                    if theme_btn.clicked() {
                        self.config.dark_mode = !self.config.dark_mode;
                        self.store_confy();
                        dbg!(self.config.dark_mode);
                    };
                })
            });

            ui.add_space(2.3);
        });
    }

    fn render_bookmarks_panel(&mut self, ctx: &Context) {

        // let side panel for the note bookmarks (for now?)
        if self.config.bookmark_panel {
            egui::SidePanel::left("left-panel!").show(ctx, |ui| {
                ui.set_min_width(140.);
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Bookmarks".to_owned()).color(
                        if self.config.dark_mode { Color32::from_rgb(100, 100, 100) } else { Color32::BLACK }).heading()
                    );
                    ui.add_space(5.);
                });
                ui.add_space(5.);

                // scroll are for the actual bookmarks
                ScrollArea::vertical().show(ui, |ui| {

                    // iterate and add the notes
                    for i in 0..self.note_warp.notes.len() {
                        let scroll_note = ui.add_enabled_ui(true, |ui| {
                            // sets the colors of the indent/separator to fit the current note
                            ui.visuals_mut().widgets.noninteractive.bg_stroke = Stroke::new(2.3, self.note_warp.notes[i].get_note_color());
                            ui.separator();
                            ui.add_space(3.);
                            // adds the note title
                            ui.indent("note_title", |ui| {
                                let title_edit = ui.text_edit_singleline(&mut self.note_warp.notes[i].title);
                                if title_edit.lost_focus() && ctx.input().key_pressed(eframe::egui::Key::Enter) {
                                    save_file("data", &mut self.note_warp.notes);
                                }
                            });
                            // adds partially the content for displa
                            let mut text = self.note_warp.notes[i].text.clone();
                            text.retain(|c| c != '\n');
                            let content = format!("{}...", text.char_range(0..80));
                            // if clicking on this, opens up a pop-up for editing the note
                            let note_btn = ui.selectable_label(false, RichText::new(content).size(13.));
                            if note_btn.clicked() {
                                self.note_warp.bool = true;
                                self.note = Some(i)
                            }
                            ui.add_space(5.);
                        });

                        scroll_note.response
                            .on_hover_text(RichText::new("A Note!"));
                    }
                });
            });
        }

    }

    fn center_panel_render(&mut self, ctx: &Context) {

        egui::CentralPanel::default()
            .frame(eframe::egui::Frame::default().fill({
                if !self.config.dark_mode {
                    Color32::from_rgb(239, 246, 255)
                } else {
                    Color32::from_rgb(20, 20, 22)
                }
            }))
            .show(ctx, |ui| {
                ui.add_space(15.);
                ui.horizontal_top(|ui| {
                   ui.add_space(15.);

                    // button for creating a new note
                    let add_note_btn = ui
                        .add(Button::new(RichText::new("+ Create Note")
                            .strong()
                            .heading()
                            .size(15.)));
                    // TODO: random id
                    if add_note_btn.clicked() {
                        self.note_warp.bool = true;
                        let new_note = Note::new(
                            1234,
                            "".to_string(),
                            "".to_string(),
                            [0, 0, 0]);
                        self.note_warp.notes.push(new_note);
                        self.note = Some(self.note_warp.notes.len() - 1);
                    }
                });

            });
    }

    fn configure_fonts(&self, ctx: &Context) {
        // create font def object
        let mut font_def = FontDefinitions::default();

        // add fonts
        font_def.font_data.insert(
            String::from("FredokaOne"),
            FontData::from_owned(
                Cow::Borrowed(include_bytes!("resources/FredokaOne-Regular.ttf")).to_vec(),
            ),
        );
        font_def.font_data.insert(
            String::from("inglobal"),
            FontData::from_owned(Cow::Borrowed(include_bytes!("resources/inglobal.ttf")).to_vec()),
        );
        // font_def
        //      .families
        //      .insert(FontFamily::Proportional, vec!["inglobal".to_string()]);

         font_def.families.get_mut(&FontFamily::Proportional).unwrap()
             .insert(0, "inglobal".to_owned());

        ctx.set_fonts(font_def);
    }

    /// Simple convenience function for quickly saving the app state.
    pub fn store_confy(&mut self) {
        confy::store(self.name(), AppConfig {
            dark_mode: self.config.dark_mode,
            bookmark_panel: self.config.bookmark_panel
        });
    }
}
