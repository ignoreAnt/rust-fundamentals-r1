fn main() {
    even_or_odd(3);
    even_or_odd(5);
    even_or_odd(6);
    even_or_odd(313123);
    even_or_odd(33432);
}

fn even_or_odd(number: i32) {
    // Use an if-else expression to determine the result.
    let result = if number % 2 == 0 {
        // If the number is even, the value of this block is "even".
        "even"
    } else {
        // If the number is odd, the value of this block is "odd".
        "odd"
    }; // The result ("even" or "odd") is assigned to the 'result' variable.

    println!("The number {} is {}.", number, result);
}
