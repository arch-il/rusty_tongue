use super::WordStatus;

#[derive(Clone)]
pub struct Translation {
    pub word: String,
    pub status: WordStatus,
}
