fn main() {
    let num:i32 = 123_456_789_0;
    println!("The large number is {num}");
    println!("The Square root of the number is : {}", num.isqrt());
    println!("The Square root of the number is : {}", num.isqrt().pow(2));
}