fn can_vote(age: u8) {
    if age >= 18 {
        println!("You are eligible to vote!");
    } else {
        println!("You are not yet eligible to vote.");
    }
}

fn main() {
    let age1 = 20;
    println!("Age: {}", age1);
    can_vote(age1);

    let age2 = 15;
    println!("Age: {}", age2);
    can_vote(age2);
}
