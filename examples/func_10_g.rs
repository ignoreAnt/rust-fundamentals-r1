fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn main() {
    let temp_c = 25.0;
    let temp_f = celsius_to_fahrenheit(temp_c);
    println!("{:.2}°C is {:.2}°F", temp_c, temp_f);
}
