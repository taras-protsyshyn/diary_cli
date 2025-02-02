use crate::models::DiaryState;
use anyhow::Result;
use std::fmt::Debug;

pub trait Database: Debug {
    fn get_state(&self) -> Result<DiaryState>;
    fn set_state(&self, new_state: DiaryState) -> Result<()>;
}

#[derive(Debug)]
pub struct JSONDatabase {
    pub path_to_data: String,
}

impl Database for JSONDatabase {
    fn get_state(&self) -> Result<DiaryState> {
        let content = std::fs::read_to_string(&self.path_to_data)?;

        let diary: DiaryState = serde_json::from_str(&content)?;

        Ok(diary)
    }

    fn set_state(&self, new_state: DiaryState) -> Result<()> {
        let content = serde_json::to_string(&new_state)?;
        std::fs::write(&self.path_to_data, content)?;

        Ok(())
    }
}
