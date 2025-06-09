fn main() {
    let point = (10, -5);
    println!("x-coordinate: {}", point.0);
    println!("y-coordinate: {}", point.1);
}





fn main() {
    let person = ("Alice", 30, "New York");
    let (name, age, city) = person;
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("City: {}", city);
}






fn main() {
    let range = 5..=10;
    println!("Range: {:?}", range.collect::<Vec<_>>());
}




fn main() {
    let mut sum = 0;
    for i in 1..=5 {
        sum += i;
    }
    println!("The sum is: {}", sum);
}




fn main() {
    let fruits = ["Mango", "Strawberry", "Pineapple"];
    for fruit in fruits {
        println!("I love {}!", fruit);
    }
}




fn main() {
    let precise_value: f64 = 123.789;
    let rounded_value: i32 = precise_value as i32;
    println!("Original float: {}", precise_value);
    println!("After casting to integer: {}", rounded_value);
}




fn main() {
    let item_count: i32 = 42;
    let item_count_float: f64 = item_count as f64;
    println!("Original integer: {}", item_count);
    println!("After casting to float: {}", item_count_float);
}




fn main() {
    let a = 5;
    let b = 10;

    dbg!(a);
    dbg!(a * b + b / 2);
}




fn main() {
    for c in 'P'..='S' {
        println!("{}", c);
    }
}




fn main() {
    let mut scores = [70, 85, 90];
    scores[1] = 88;
    println!("{:?}", scores);
}




fn main() {
    let my_tuple = ('R', 2024, true);
    println!("{:#?}", my_tuple);
}




fn main() {
    let value = 100;

    let in_range = value > 50 && value < 150;
    println!("Is value in range (50, 150)? {}", in_range);

    let result = value as f64 / 3.0;
    println!("Result after casting and dividing: {:.2}", result);
}




