fn calculate_area(radius: f64) -> f64 {
    let pi = 3.14159;
    pi * radius.powi(2) // Implicit return
}

fn main() {
    let radius = 5.0;
    let area = calculate_area(radius);
    println!("The area of a circle with radius {} is {:.2}", radius, area);
}
