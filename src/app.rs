use std::borrow::Cow;

use eframe::epi;
use egui::{Color32, CtxRef, FontData, FontDefinitions, FontFamily, RichText, Ui, Layout, TextStyle, Label, Button, Visuals};


const SEMI_WHITE: Color32 = Color32::from_rgb(200, 255, 255);

pub struct HeadlinesConfig {
    theme_mode: bool,
}

impl HeadlinesConfig {
    fn new() -> Self {
        HeadlinesConfig {
            theme_mode: true,
        }
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct SnowApp {
    label: String,
    empty_label: String,
    config: HeadlinesConfig,
}

impl Default for SnowApp {
    fn default() -> Self {
        Self {
            label: "Hello snowy world!".to_owned(),
            empty_label: "".to_owned(),
            config: HeadlinesConfig::new(),
        }
    }
}

impl epi::App for SnowApp {
    // everything here gets rendered all the time
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {

        if self.config.theme_mode {
            ctx.set_visuals(Visuals::dark())
        } else {
            ctx.set_visuals(Visuals::light())
        }

        self.render_top_panel(ctx, frame);

        egui::SidePanel::left("left-panel!").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.colored_label(
                    if self.config.theme_mode {Color32::LIGHT_BLUE} else {Color32::BLACK},
                    "Bookmarks");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.config.theme_mode {
                ui.visuals_mut().extreme_bg_color = Color32::LIGHT_BLUE;
            }
            ui.heading(RichText::new("Hello snowy world!").color(
                match self.config.theme_mode {
                    true => Color32::WHITE,
                    false => Color32::BLACK,
            }));
            ui.add_space(10.);
            self.url_input(ctx, ui);
        });
    }

    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        self.configure_fonts(ctx);
    }

    fn name(&self) -> &str {
        "Snow's Window"
    }
}

impl SnowApp {
    fn render_top_panel(&mut self, ctx: &CtxRef, frame: &epi::Frame) {
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.add_space(4.);

            egui::menu::bar(ui, |ui| {
                //add the snow-symbol to the top bar
                ui.with_layout(Layout::left_to_right(), |ui| {
                    ui.add(Label::new(
                        RichText::new("❄")
                            .text_style(TextStyle::Heading)
                            .strong()));
                });
                // add the closing, menu and theme button
                ui.with_layout(Layout::right_to_left(), |ui| {
                    let close_btn = ui.add(Button::new(
                       RichText::new("X")
                           .text_style(TextStyle::Heading)
                           .strong()));
                    let config_btn = ui.add(Button::new(
                        RichText::new("C")
                            .text_style(TextStyle::Heading)
                            .strong()));
                    let theme_btn = ui.add(Button::new(
                        RichText::new("T")
                            .text_style(TextStyle::Heading)
                            .strong()));

                    // add logic to the close button
                    if close_btn.clicked() {
                        dbg!("Closing app!");
                        frame.quit();
                    };

                    //add logic to the theme button
                    if theme_btn.clicked() {
                        self.config.theme_mode = !self.config.theme_mode;
                        dbg!(self.config.theme_mode);
                    };
                })
            });

            ui.add_space(4.);
        });
    }

    // url input func for setting the file to parse
    fn url_input(&mut self, ctx: &CtxRef, ui: &mut Ui) {
        let Self {
            label: _,
            empty_label,
            config: _
        } = self;
        let input = ui.text_edit_singleline(empty_label);

        if input.lost_focus() && ctx.input().key_pressed(eframe::egui::Key::Enter) {
            ui.add_space(10.);
            ui.add(egui::Label::new(self.empty_label.as_str()));
        }
    }

    fn configure_fonts(&self, ctx: &CtxRef) {
        // create font def object
        let mut font_def = FontDefinitions::default();
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
        // set font size
        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Heading,
            (FontFamily::Proportional, 20.),
        );

        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Body,
            (FontFamily::Proportional, 17.),
        );

        // set the font for the Proportional family
        font_def
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "inglobal".to_string());

        ctx.set_fonts(font_def);
    }
}
