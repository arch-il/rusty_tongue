use super::WordStatus;

#[derive(Clone)]
pub struct Translation {
    pub from: String,
    pub to: String,
    pub status: WordStatus,
}
