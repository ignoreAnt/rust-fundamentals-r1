fn main() {

    let is_sunny = true;
    let is_weekend = false;

    println!("Sunny weekend: {}", is_sunny && is_weekend); // false

    println!("Sunny or weekend: {}", is_sunny || is_weekend); // true

    println!("Not sunny: {}", !is_sunny); // false
}
