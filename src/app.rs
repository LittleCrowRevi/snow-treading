use std::borrow::Cow;

use eframe::egui::Direction;
use eframe::epi;
use egui::{
    Button, Color32, CtxRef, FontData, FontDefinitions, FontFamily, Frame, Label, Layout, RichText,
    TextStyle, Ui, Visuals,
};
use epi::App;

const SEMI_WHITE: Color32 = Color32::from_rgb(200, 255, 255);

// simple config struct
pub struct AppConfig {
    dark_mode: bool,
}

impl AppConfig {
    fn new() -> Self {
        AppConfig { dark_mode: true }
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
}

impl Default for SnowApp {
    fn default() -> Self {
        Self {
            label: "Hello snowy world!".to_owned(),
            empty_label: "".to_owned(),
            config: AppConfig::new(),
        }
    }
}

impl App for SnowApp {
    // everything here gets rendered all the time
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
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
                visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(255, 251, 240);
                ctx.set_visuals(visuals);
            }
        }

        // call top panel render
        self.render_top_panel(ctx, frame);

        egui::SidePanel::left("left-panel!").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.colored_label(
                    if self.config.dark_mode {
                        Color32::LIGHT_BLUE
                    } else {
                        Color32::BLACK
                    },
                    "Bookmarks",
                );
            });
        });

        self.center_panel_render(ctx);
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
            ui.add_space(3.3);

            egui::menu::bar(ui, |ui| {
                ui.min_size().x = 20.;

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
                            RichText::new("Hello snowy world!")
                                .color(match self.config.dark_mode {
                                    true => Color32::WHITE,
                                    false => Color32::BLACK,
                                })
                                .strong(),
                        ))
                    },
                );

                // add the closing, menu and theme button
                ui.with_layout(Layout::right_to_left(), |ui| {
                    let close_btn = ui.add(Button::new(RichText::new("X").strong()));
                    let config_btn = ui.add(Button::new(RichText::new("C").strong()));
                    let theme_btn = ui.add(Button::new(RichText::new("T").strong()));

                    // add logic to the close button
                    if close_btn.clicked() {
                        dbg!("Closing app!");
                        frame.quit();
                    };

                    //add logic to the theme button
                    if theme_btn.clicked() {
                        self.config.dark_mode = !self.config.dark_mode;
                        dbg!(self.config.dark_mode);
                    };
                })
            });

            ui.add_space(2.6);
        });
    }

    fn center_panel_render(&mut self, ctx: &CtxRef) {
        egui::CentralPanel::default()
            .frame(Frame::default().fill({
                if !self.config.dark_mode {
                    Color32::from_rgb(239, 246, 255)
                } else {
                    Color32::from_rgb(20, 20, 22)
                }
            }))
            .show(ctx, |ui| {
                ui.add_space(5.);
                ui.indent("", |ui| ui.add(egui::widgets::ProgressBar::new(10.)));

                ui.add(egui::Label::new(
                    RichText::new("Hello snowy world!")
                        .color(match self.config.dark_mode {
                            true => Color32::WHITE,
                            false => Color32::BLACK,
                        })
                        .heading(),
                ));

                ui.add_space(10.);
                // self.url_input(ctx, ui);
            });
    }

    // url input func for setting the file to parse
    fn url_input(&mut self, ctx: &CtxRef, ui: &mut Ui) {
        let Self {
            label: _,
            empty_label,
            config: _,
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
