fn main() {
    let first_initial = 'M'; // This is a char
    let another_char: char = 'Z'; // With explicit type annotation
    println!("Initial: {}, Another: {}", first_initial, another_char);

    let emoji_char = '🥱'; // Example: headphones emoji
    println!("My favorite emoji character: {}", emoji_char);
    println!("My favorite emoji character is alphabetic: {}", emoji_char.is_alphabetic());
    println!("first_initial character is alphabetic: {}", first_initial.is_alphabetic());
}