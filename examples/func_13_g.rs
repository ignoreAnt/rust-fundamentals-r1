fn calculate_rectangle_area(dimensions: (f64, f64)) -> f64 {
    dimensions.0 * dimensions.1
}

fn main() {
    let rect = (5.0, 10.0);
    let area = calculate_rectangle_area(rect);
    println!("The area of the rectangle is: {}", area);
}
