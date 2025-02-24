use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Savestate {
    pub page_location: usize,
    pub word_location: usize,
    pub translate_history: Vec<String>,
}
