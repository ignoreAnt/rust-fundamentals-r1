fn is_longer(s1: &str, s2: &str) -> bool {
    s1.len() > s2.len()
}

fn main() {
    let string1 = "Hello, world!";
    let string2 = "Rust";

    let result = is_longer(string1, string2);
    println!("Is '{}' longer than '{}'? {}", string1, string2, result);
}
