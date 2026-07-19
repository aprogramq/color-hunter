use users::get_current_username;

pub fn get_username() -> String {
    get_current_username()
        .expect("Failed to get current username")
        .into_string()
        .expect("Failed convert to username to string")
}
