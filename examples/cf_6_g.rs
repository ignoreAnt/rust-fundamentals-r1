fn classify_number(number: i32) -> &str {
    if number > 0 {
        "positive"
    } else if number < 0 {
        "negative"
    } else {
        "zero"
    }
}

fn main() {
    let n1 = 5;
    let n2 = -3;
    let n3 = 0;

    println!("{} is {}", n1, classify_number(n1));
    println!("{} is {}", n2, classify_number(n2));
    println!("{} is {}", n3, classify_number(n3));
}
