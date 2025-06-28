fn get_meal(day: &str) -> &str {
    match day {
        "Monday" => "Spaghetti",
        "Wednesday" => "Chicken",
        "Saturday" | "Sunday" => "Pizza",
        _ => "Salad", // Catch-all for all other inputs
    }
}

fn main() {
    let days = ["Monday", "Tuesday", "Wednesday", "Saturday", "Friday", "Sunday"];
    
    for day in days.iter() {
        println!("Meal on {}: {}", day, get_meal(day));
    }
}
