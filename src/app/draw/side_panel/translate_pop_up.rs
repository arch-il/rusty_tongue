use eframe::egui;

use super::Language;

pub struct TranslatePopUp {
    pub open: bool,
    pub id: egui::Id,
    pub language_from: Language,
    pub language_to: Language,
    pub text_from: String,
    pub text_to: String,
}

impl TranslatePopUp {
    pub fn new() -> Self {
        Self {
            open: false,
            id: egui::Id::new("translate id"),
            language_from: Language::German,
            language_to: Language::English,
            text_from: String::new(),
            text_to: String::new(),
        }
    }
}
