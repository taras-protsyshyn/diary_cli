use chrono::NaiveDate;

use crate::{
    db::{Database, JSONDatabase},
    models::Note,
};

#[derive(Debug)]
pub struct Diary {
    pub db: Box<dyn Database>,
}

impl Diary {
    pub fn new(path_to_data: String) -> Self {
        Self {
            db: Box::new(JSONDatabase { path_to_data }),
        }
    }

    pub fn add_note(&mut self, content: String, date: NaiveDate) -> Result<(), anyhow::Error> {
        let mut state: crate::models::DiaryState = self.db.get_state()?;
        state
            .notes
            .insert(date.format("%Y-%m-%d").to_string(), Note::new(content));
        self.db.set_state(state)?;

        Ok(())
    }
}
