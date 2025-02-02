use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Note {
    content: String,
}

impl Note {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct DiaryState {
    count: u32,
    pub notes: HashMap<String, Note>,
}
