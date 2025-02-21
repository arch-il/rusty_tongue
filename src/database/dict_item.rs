#[derive(Clone)]
pub struct DictItem {
    pub left_word: String,
    pub right_word: String,
    pub classes: Vec<String>,
    pub genders: Vec<String>,
}
