fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}

fn main() {
    let test_chars = ['a', 'b', 'e', 'z', 'i'];

    for ch in test_chars {
        println!("Is '{}' a vowel? {}", ch, is_vowel(ch));
    }
}
