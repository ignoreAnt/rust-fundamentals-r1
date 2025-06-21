fn is_negative(number: i32) -> bool {
    number < 0
}

fn main() {
    let number = -5;
    println!("Is {} negative? {}", number, is_negative(number));

    let number = 8;
    println!("Is {} negative? {}", number, is_negative(number));
}
