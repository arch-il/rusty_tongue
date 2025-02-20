use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Savestate {
    pub location: usize,
    pub translate_history: Vec<String>,
}
