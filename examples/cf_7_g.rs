fn day_of_week(day: u8) -> &str {
    match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid day",
    }
}

fn main() {
    for day in 0..9 {
        println!("Day {}: {}", day, day_of_week(day));
    }
}
