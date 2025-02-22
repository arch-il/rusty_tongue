use eframe::egui::{self, Color32, RichText, Sense, Ui, Vec2, Widget};

use crate::{
    app::MyEguiApp,
    database::{DictItem, WordStatus},
};

pub struct DictionaryPopUp {
    pub open: bool,
    pub id: egui::Id,
    pub search_text: String,
    pub filter: (bool, bool, bool),
    pub curr_word: Option<String>,
    pub curr_entries: Option<Vec<DictItem>>,
    pub entry_id: egui::Id,
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
            entry_id: egui::Id::new("entry id"),
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
        // ! this is so stupid solution. TRY TO FIX THIS LATER
        let mut word = None;

        egui::Window::new(RichText::from("Dictionary").strong())
            .open(&mut self.dictionary_pop_up.open)
            .resizable(true)
            .max_width(200.0)
            .max_height(200.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::from("Search:").strong());

                    egui::TextEdit::singleline(&mut self.dictionary_pop_up.search_text)
                        .id(self.dictionary_pop_up.id)
                        .show(ui);

                    if ui.button("❌").clicked() {
                        self.dictionary_pop_up.search_text = String::new();
                    }
                });

                ui.horizontal(|ui| {
                    ui.label(RichText::from("Filter:").strong());

                    if ui
                        .selectable_label(
                            self.dictionary_pop_up.filter.0,
                            RichText::from("❌").color(Color32::LIGHT_RED),
                        )
                        .clicked()
                    {
                        self.dictionary_pop_up.filter.0 = !self.dictionary_pop_up.filter.0;
                    }
                    if ui
                        .selectable_label(
                            self.dictionary_pop_up.filter.1,
                            RichText::from("📖").color(Color32::LIGHT_YELLOW),
                        )
                        .clicked()
                    {
                        self.dictionary_pop_up.filter.1 = !self.dictionary_pop_up.filter.1;
                    }
                    if ui
                        .selectable_label(
                            self.dictionary_pop_up.filter.2,
                            RichText::from("✅").color(Color32::LIGHT_GREEN),
                        )
                        .clicked()
                    {
                        self.dictionary_pop_up.filter.2 = !self.dictionary_pop_up.filter.2;
                    }
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
                        .database
                        .search_user_entries(&self.dictionary_pop_up.search_text)
                        .iter()
                        .filter(|t| status_filter.contains(&t.status))
                        .rev()
                    {
                        ui.horizontal(|ui| {
                            match t.status {
                                WordStatus::NotAWord => {
                                    ui.label(RichText::from("❌").color(Color32::LIGHT_RED))
                                }

                                WordStatus::Learning => {
                                    ui.label(RichText::from("📖").color(Color32::LIGHT_YELLOW))
                                }
                                WordStatus::Mastered => {
                                    ui.label(RichText::from("✅").color(Color32::LIGHT_GREEN))
                                }
                            };

                            let label_button =
                                egui::Label::new(&t.word).sense(Sense::click()).ui(ui);
                            if label_button.clicked() {
                                word = Some(t.word.clone());
                            }
                        });
                    }
                })
            });

        if let Some(word) = word {
            self.set_entry_pop_up_word(&word);
        }
    }

    pub fn set_entry_pop_up_word(&mut self, word: &str) {
        // ! try to remove clones
        if self.dictionary_pop_up.curr_word != Some(String::from(word)) {
            self.dictionary_pop_up.curr_word = Some(String::from(word));
            self.dictionary_pop_up.curr_entries = None;
        }
    }

    pub fn word_entry_pop_up(&mut self, ctx: &egui::Context) {
        let mut open;
        let word = if let Some(word) = &self.dictionary_pop_up.curr_word {
            open = true;
            word
        } else {
            return;
        };

        egui::Window::new(RichText::from(format!("Word Entry: {word}")).strong())
            .id(self.dictionary_pop_up.entry_id)
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
                    let temp = self.database.search_dict_entries(word);
                    self.dictionary_pop_up.curr_entries = Some(temp);
                    &self.dictionary_pop_up.curr_entries.clone().unwrap() // ! annyoing clone here
                };

                for entry in entries {
                    ui.horizontal(|ui| {
                        ui.label(
                            format!("{:?}", entry.classes)
                                .chars()
                                .into_iter()
                                .filter(|c| c != &'\"')
                                .collect::<String>(),
                        );
                        ui.label(
                            format!("{:?}", entry.genders)
                                .chars()
                                .into_iter()
                                .filter(|c| c != &'\"')
                                .collect::<String>(),
                        );

                        ui.label(RichText::from(&entry.left_word).strong());
                        ui.label("-");
                        ui.label(RichText::from(&entry.right_word).strong());

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
