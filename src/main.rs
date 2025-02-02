use chrono::NaiveDate;

use dotenv::dotenv;

mod db;
mod diary;
mod io_utils;
mod models;

fn main() {
    dotenv().ok();

    let mut diary =
        diary::Diary::new(std::env::var("PATH_TO_DATA").expect("PATH_TO_DATA must be set"));

    let user_input = io_utils::get_user_input();

    diary
        .add_note(
            user_input.to_owned(),
            NaiveDate::from_ymd_opt(2021, 10, 11)
                .expect("Can't create NaiveDate from year, month, and day"),
        )
        .expect("Can't add note");

    println!("You entered: {}", user_input);
    print!(
        "Diary: {:?}",
        diary.db.get_state().expect("Can't get state")
    );
}
