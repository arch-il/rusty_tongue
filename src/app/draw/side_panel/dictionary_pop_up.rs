use dictcc::DictEntry;
use eframe::egui::{self, Color32, RichText, Sense, Ui, Vec2, Widget};

use crate::{
    app::{text_utils, MyEguiApp},
    database::WordStatus,
};

pub struct DictionaryPopUp {
    pub open: bool,
    pub id: egui::Id,
    pub search_text: String,
    pub filter: (bool, bool, bool),
    pub curr_word: Option<String>,
    pub curr_entries: Option<Vec<DictEntry>>,
    // pub hide_translated: bool,
}

impl DictionaryPopUp {
    pub fn new() -> Self {
        Self {
            open: false,
            search_text: String::new(),
            id: egui::Id::new("dictionary search id"),
            filter: (false, true, true),
            curr_word: None,
            curr_entries: None,
            // hide_translated: false,
        }
    }
}

impl MyEguiApp {
    pub fn open_dictionary_button(&mut self, ui: &mut Ui) {
        if ui
            .selectable_label(self.dictionary_pop_up.open, "Open Dictionary")
            .clicked()
        {
            self.toggle_dictionary_pop_up(ui.ctx());
        }
    }

    pub fn toggle_dictionary_pop_up(&mut self, ctx: &egui::Context) {
        self.dictionary_pop_up.open = !self.dictionary_pop_up.open;
        if self.dictionary_pop_up.open {
            ctx.memory_mut(|mem| mem.request_focus(self.dictionary_pop_up.id));
        }
    }

    pub fn dictionary_pop_up(&mut self, ctx: &egui::Context) {
        egui::Window::new("Dictionary")
            .open(&mut self.dictionary_pop_up.open)
            .resizable(true)
            .max_width(200.0)
            .max_height(200.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Search:");

                    egui::TextEdit::singleline(&mut self.dictionary_pop_up.search_text)
                        .id(self.dictionary_pop_up.id)
                        .show(ui);

                    if ui.button("❌").clicked() {
                        self.dictionary_pop_up.search_text = String::new();
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Filter:");

                    if ui
                        .selectable_label(self.dictionary_pop_up.filter.0, "❌")
                        .clicked()
                    {
                        self.dictionary_pop_up.filter.0 = !self.dictionary_pop_up.filter.0;
                    }
                    if ui
                        .selectable_label(
                            self.dictionary_pop_up.filter.1,
                            RichText::from("📖").color(Color32::YELLOW),
                        )
                        .clicked()
                    {
                        self.dictionary_pop_up.filter.1 = !self.dictionary_pop_up.filter.1;
                    }
                    if ui
                        .selectable_label(
                            self.dictionary_pop_up.filter.2,
                            RichText::from("✅").color(Color32::GREEN),
                        )
                        .clicked()
                    {
                        self.dictionary_pop_up.filter.2 = !self.dictionary_pop_up.filter.2;
                    }

                    // ui.add_space(10.0);
                    // ui.label("Hide:");
                    // ui.checkbox(&mut self.dictionary_pop_up.hide_translated, "");
                });

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.allocate_at_least(
                        Vec2 {
                            x: ui.available_width(),
                            y: 0.0,
                        },
                        Sense::empty(),
                    );

                    // ? maybe move this to SQL
                    let mut status_filter = Vec::new();
                    if self.dictionary_pop_up.filter.0 {
                        status_filter.push(WordStatus::NotAWord);
                    }
                    if self.dictionary_pop_up.filter.1 {
                        status_filter.push(WordStatus::Learning);
                    }
                    if self.dictionary_pop_up.filter.2 {
                        status_filter.push(WordStatus::Mastered);
                    }

                    for t in self
                        .user_database
                        .get_by_search(&self.dictionary_pop_up.search_text)
                        .iter()
                        .filter(|t| status_filter.contains(&t.status))
                        .rev()
                    {
                        ui.horizontal(|ui| {
                            match t.status {
                                WordStatus::NotAWord => ui.label("❌"),
                                WordStatus::Learning => {
                                    ui.label(RichText::from("📖").color(Color32::YELLOW))
                                }
                                WordStatus::Mastered => {
                                    ui.label(RichText::from("✅").color(Color32::GREEN))
                                }
                                _ => panic!("Invalid status in database"),
                            };

                            let label_button =
                                egui::Label::new(&t.word).sense(Sense::click()).ui(ui);
                            if label_button.clicked() {
                                // ! try to remove clones
                                if self.dictionary_pop_up.curr_word != Some(t.word.clone()) {
                                    self.dictionary_pop_up.curr_word = Some(t.word.clone());
                                    self.dictionary_pop_up.curr_entries = None;
                                }
                            }
                            // ui.label(&t.word);
                            // ui.label("-");
                            // let translated =
                            //     text_utils::find_in_dict(&self.dict_database, &t.word).unwrap();
                            // ui.label(translated.right_word.plain_word());

                            // ! enable later
                            // if !self.dictionary_pop_up.hide_translated {
                            //     ui.label("-");
                            //     ui.label(&t.to);
                            // }
                        });
                    }
                })
            });
    }

    pub fn word_entry_pop_up(&mut self, ctx: &egui::Context) {
        let mut open;
        let word = if let Some(word) = &self.dictionary_pop_up.curr_word {
            open = true;
            word
        } else {
            return;
        };

        egui::Window::new(format!("Word Entry: {word}"))
            .open(&mut open)
            .resizable(true)
            .max_width(350.0)
            .max_height(250.0)
            .vscroll(true)
            .hscroll(true)
            .show(ctx, |ui| {
                let entries = if let Some(entries) = &self.dictionary_pop_up.curr_entries {
                    entries
                } else {
                    let temp = text_utils::find_in_dict(&self.dict_database, word).unwrap();
                    // .rev()
                    self.dictionary_pop_up.curr_entries = Some(temp);
                    &self.dictionary_pop_up.curr_entries.clone().unwrap() // ! annyoing clone here
                };

                for entry in entries {
                    ui.horizontal(|ui| {
                        if !entry.word_classes.is_empty() {
                            ui.label(format!("{:?}", entry.word_classes));
                        }
                        if !entry.left_word.genders().is_empty() {
                            ui.label(format!("{:?}", entry.left_word.genders()));
                        }

                        ui.label(RichText::from(entry.left_word.plain_word()).strong());
                        ui.label("-");
                        ui.label(RichText::from(entry.right_word.plain_word()).strong());

                        // ? maybe enable in the future
                        // ui.label(format!("{:?}", entry.left_word.word_with_optional_parts()));
                        // ui.label(format!("{:?}", entry.left_word.acronyms()));
                        // ui.label(format!("{:?}", entry.left_word.comments()));
                    });
                }
            });

        if !open {
            self.dictionary_pop_up.curr_word = None;
            self.dictionary_pop_up.curr_entries = None;
        }
    }
}
