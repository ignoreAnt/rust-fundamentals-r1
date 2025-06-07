fn main() {
    let a = 2;
    let b = 3;
    // dbg! prints info about (a + b) and then returns its value (5)
    let result = dbg!(a + b) * 10;
    println!("(a + b) * 10 = {}", (a + b) * 10);
    println!("Final result: {}", result); // result will be 50

    let seasons = ["Spring", "Summer", "Fall", "Winter"];
    dbg!(seasons);
}