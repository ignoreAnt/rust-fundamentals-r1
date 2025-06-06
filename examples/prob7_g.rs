fn main() {
    
    let mut score = 100;
    println!("Initial score: {}", score);

    score += 50;
    println!("After += 50: {}", score);

    score -= 20;
    println!("After -= 20: {}", score);

    score *= 2;
    println!("After *= 2: {}", score);

    score /= 4;
    println!("After /= 4: {}", score);
}
