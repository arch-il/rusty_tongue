#[allow(unused)] // ! remove when these are integrated in pop_up
#[derive(Clone)]
pub struct DictItem {
    pub classes: Vec<String>,
    pub left_word: String,
    pub left_genders: Vec<String>,
    pub left_acronyms: Vec<String>,
    pub left_comments: Vec<String>,
    pub right_word: String,
    pub right_genders: Vec<String>,
    pub right_acronyms: Vec<String>,
    pub right_comments: Vec<String>,
}
