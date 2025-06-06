fn main() {

    let my_char: char = '7';

    println!("Is '{}' alphabetic? {}", my_char, my_char.is_alphabetic());

    println!("Is '{}' numeric? {}", my_char, my_char.is_numeric());

    let letter_char: char = 'P';

    println!("Is '{}' uppercase? {}", letter_char, letter_char.is_uppercase());
}
