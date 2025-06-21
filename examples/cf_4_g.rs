fn minutes_to_seconds(minutes: u32) -> u32 {
    minutes * 60
}

fn main() {
    let minutes = 5;
    let seconds = minutes_to_seconds(minutes);
    println!("{} minutes is {} seconds.", minutes, seconds);
}
