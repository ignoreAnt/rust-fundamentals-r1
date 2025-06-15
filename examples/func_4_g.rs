fn multiply(a: i32, b: i32) -> i32 {
    if a == 0 || b == 0{
       return   0
    }
    a * b
}

fn main() {
    let result = multiply(6, 7);
    println!("The product is: {}", result);
}
