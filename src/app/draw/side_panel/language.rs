#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Language {
    English,
    German,
    Italian,
}

pub fn language_to_string(language: Language) -> String {
    String::from(match language {
        Language::English => "Engilish",
        Language::German => "German",
        Language::Italian => "Italian",
    })
}

pub fn language_to_code(language: Language) -> String {
    String::from(match language {
        Language::English => "en",
        Language::German => "de",
        Language::Italian => "it",
    })
}
