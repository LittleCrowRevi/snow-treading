use std::borrow::Cow;

use eframe::epi;
use egui::{Color32, CtxRef, FontData, FontDefinitions, FontFamily, RichText, Ui};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct SnowApp {
    label: String,
    empty_label: String,
}

impl Default for SnowApp {
    fn default() -> Self {
        Self {
            label: "Hello snowy world!".to_owned(),
            empty_label: "".to_owned(),
        }
    }
}

impl epi::App for SnowApp {
    fn name(&self) -> &str {
        "Snow's Window"
    }

    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        let Self {
            label: _,
            empty_label: _,
        } = self;

        egui::SidePanel::left("Left Panel!").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.colored_label(Color32::LIGHT_BLUE, "Bookmarks");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(RichText::new("Hello snowy world!").color(Color32::WHITE));
            ui.add_space(10.);
            self.url_input(ctx, ui);
        });
    }
}

impl SnowApp {
    fn url_input(&mut self, ctx: &CtxRef, ui: &mut Ui) {
        let Self {
            label: _,
            empty_label,
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
        font_def
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "inglobal".to_string());

        ctx.set_fonts(font_def);
    }
}
