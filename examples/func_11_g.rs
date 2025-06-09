fn get_user_profile() -> (i32, &'static str, bool) {
    (1337, "rustacean", true)
}

fn main() {
    let profile = get_user_profile();
    println!("User ID: {}", profile.0);
    println!("Username: {}", profile.1);
    println!("Is active: {}", profile.2);
}
