use std::io;

pub fn get_user_input() -> String {
    let mut user_input = String::new();

    let _ = io::stdin().read_line(&mut user_input);

    user_input
}
