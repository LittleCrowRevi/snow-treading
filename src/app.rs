use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use eframe::egui::{Button, Color32, Context, Direction, FontData, FontDefinitions, FontFamily, Frame, Label, Layout, RichText, TextStyle, TopBottomPanel, Ui, Visuals, FontId, TextBuffer, Stroke, Vec2, Rgba};
use eframe::epi;
use epi::App;
use egui::{Slider, ScrollArea};
use eframe::epi::Storage;
use std::time::Duration;
use snow_treading::{Note, load_file, save_file};


// simple config struct
#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    dark_mode: bool,
}

impl AppConfig {
    fn new() -> Self {
        AppConfig { dark_mode: true }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            dark_mode: true
        }
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct SnowApp {
    label: String,
    // an empty label for text inputs?
    empty_label: String,
    config: AppConfig,
    progress_bar: f32,
    notes: Vec<Note>,
}

impl App for SnowApp {

    // everything here gets rendered all the time
    fn update(&mut self, ctx: &Context, frame: &epi::Frame) {
        // create Visuals object
        // inherits all the default values from dark and light based on theme mode
        // ease of changing default visuals this way
        let mut visuals = Visuals {

            ..if self.config.dark_mode {Visuals::dark()} else {Visuals::light()}
        };

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

    fn on_exit(&mut self) {
    }

    fn name(&self) -> &str {
        "Snow's Window"
    }

    fn auto_save_interval(&self) -> Duration {
        Duration::new(300, 0)
    }

}

impl SnowApp {

    pub fn new() -> SnowApp {

        let config: AppConfig = confy::load("SnowApp").unwrap_or_default();

        SnowApp {
            label: "Hello snowy world!".to_owned(),
            empty_label: "".to_owned(),
            config,
            progress_bar: 0.0,
            notes: load_file("data")
        }
    }

    fn render_top_panel(&mut self, ctx: &Context, frame: &epi::Frame) {
        TopBottomPanel::top("header").show(ctx, |ui| {
            ui.add_space(3.3);

            egui::menu::bar(ui, |ui| {

                //add the snow-symbol to the top bar
                ui.with_layout(Layout::left_to_right(), |ui| {
                    ui.add(Label::new(
                        RichText::new("❄").text_style(TextStyle::Heading).strong(),
                    ));
                });

                // add top bar title string
                ui.with_layout(
                    Layout::centered_and_justified(Direction::LeftToRight),
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
                    let close_btn = ui
                        .add(Button::new(RichText::new("⛔  ").heading()))
                        .on_hover_text(RichText::new("Exit App"));
                    let config_btn = ui
                        .add(Button::new(
                            RichText::new("⚙︎")
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
                        confy::store("SnowApp", AppConfig {
                            dark_mode: self.config.dark_mode
                        });
                        dbg!(self.config.dark_mode);
                    };
                })
            });

            ui.add_space(2.3);
        });
    }

    fn render_bookmarks_panel(&mut self, ctx: &Context) {

        // let side panel for the note bookmarks (for now?)
        egui::SidePanel::left("left-panel!").show(ctx, |ui| {
            ui.set_min_width(140.);

            ui.horizontal(|ui| {
                ui.label(RichText::new("Bookmarks".to_owned()).color(
                    if self.config.dark_mode { Color32::from_rgb(100, 100, 100) } else { Color32::BLACK }).heading()
                )
            });
            ui.add_space(5.);

            // scroll are for the actual bookmarks
            ScrollArea::vertical().show(ui, |ui| {

                // iterate and add the notes
                for i in 0..self.notes.len() {
                    let scroll_note = ui.add_enabled_ui(true, |ui| {
                        // sets the colors of the indent/separator to fit the current note
                        ui.visuals_mut().widgets.noninteractive.bg_stroke = Stroke::new(2.3, self.notes[i].get_note_color());

                        ui.separator();
                        ui.add_space(3.);
                        // adds the note title
                        ui.indent("note_title", |ui| {
                            let title_edit = ui.text_edit_singleline(&mut self.notes[i].title);
                            if title_edit.lost_focus() && ctx.input().key_pressed(eframe::egui::Key::Enter) {
                                save_file("data", &mut self.notes);
                            }
                        });
                        // adds partially the content for display
                        let content = format!("{}...", self.notes[i].text.char_range(0..80));
                        let note_window = ui.selectable_label(false, RichText::new(content).size(13.));

                        ui.add_space(5.);
                    });

                    scroll_note.response
                        .on_hover_text(RichText::new("A Note!"));
                }
            });
        });
    }

    fn center_panel_render(&mut self, ctx: &Context) {

        egui::CentralPanel::default()
            .frame(Frame::default().fill({
                if !self.config.dark_mode {
                    Color32::from_rgb(239, 246, 255)
                } else {
                    Color32::from_rgb(20, 20, 22)
                }
            }))
            .show(ctx, |ui| {

                // padding
                ui.add_space(5.);

                // simple progress bar for test purposes
                let bar = egui::widgets::ProgressBar::new(self.progress_bar).animate(true);
                ui.indent("", |ui| ui.add(bar));
                ui.add_space(10.);
                // button to increment the bar
                let progress_btn = ui.add(Button::new(RichText::new("+10%")));
                if progress_btn.clicked() {
                    self.progress_bar += 0.1;
                }
                // slider for the progress bar
                let slider = ui.add(Slider::new(&mut self.progress_bar, 0.0..=1.0));
                // self.url_input(ctx, ui);
            });
    }

    fn note_window(&mut self, ctx: Context, ui: &mut Ui) {


    }

    // url input func for setting the file to parse
    fn url_input(&mut self, ctx: &Context, ui: &mut Ui) {
        let Self {
            label: _,
            empty_label,
            config: _, ..
        } = self;
        let input = ui.text_edit_singleline(empty_label);

        if input.lost_focus() && ctx.input().key_pressed(eframe::egui::Key::Enter) {
            ui.add_space(10.);
            ui.add(egui::Label::new(self.empty_label.as_str()));
        }
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
}
