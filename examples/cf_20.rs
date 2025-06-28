fn is_empty(input: &str) -> bool {
    input.len() == 0
}

fn main() {
    let s1 = "";
    let s2 = "Hello";

    println!("Is s1 empty? {}", is_empty(s1));
    println!("Is s2 empty? {}", is_empty(s2));
}
