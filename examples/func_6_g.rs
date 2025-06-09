fn invert_bool(value: bool) -> bool {
    !value
}

fn main() {
    let original = true;
    let inverted = invert_bool(original);
    println!("Original: {}, Inverted: {}", original, inverted);
}
