fn main() {
    let temperature_celsius: f64 = 25.5;

    let is_comfortable = temperature_celsius >= 20.0 && temperature_celsius <= 26.0;

    println!("Is the temperature comfortable? {}", is_comfortable);
}
