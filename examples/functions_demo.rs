use std::ops::Range;

fn main() {
    let apples = 50;
    
    
    open_store("WhiteField");
    open_store("University");
    open_store("Community Center");
    open_store("Times Square");
    
    
    let toppings = ["cheese", "chicken", "capsicum"];
    let toppings2 = ["cheese", "chicken"];
    
    bake_pizza(2, &toppings);
    
    bake_pizza(3, &toppings2);
    // bake_pizza(4, "capsicum");
    
    // println!("After lunch");
    // swim_in_profit();
    // 
    // bake_pizza();
    // bake_pizza();
    // bake_pizza();
    // 
    // println!("before closing the store");
    // swim_in_profit();
    // 
    // close_store();

    let return_value = my_function();
    println!("my_function : {}", return_value);

    let return_value = my_function_v1();
    println!("my_function_v1 : {}", return_value);

    let add_result = add(34656, 97754);
    println!("add_result is {add_result}");

    println!("{}", square(4));
    println!("{}", square(8));
    println!("{}", cube(3));
    println!("{}", cube_v1(5));
    
    println!("{}", sum_range(1..68));
    println!("{}", sum_range(1..5));
    
    let nums = [11, 2, 3, 4, 5, 6, 7];
    println!("{}", sum_array(nums));
    println!("{}", sum_array(nums));
    
    let mys = mystery();
    println!("mys : {:?}", mys);

    println!("close_store : {:?}", close_store());
    
    let miles = {
        let kms = 10.0;
        kms * 1.6
    };
    
    println!("miles : {}", miles);
    println!("{}", is_even(5));
    println!("{}", is_even(10));

    println!("Contains 'a' and 'v' in 'aardvark': {:?}", 
             has_chars("aardvark", 'a', 'v'));
    println!("Contains 'z' and 'y' in 'zoology': {:?}", 
             has_chars("zoology", 'z', 'y'));
    println!("Contains 'z' and 'c' in 'zebra': {:?}", 
             has_chars("zebra", 'e', 'c'));

    println!("Contains 'a' in 'aardvark': {:?}",
             has_chars_with_len("aardvark", 'a'));
    
}

fn is_even(number: i32) -> bool {
    // The '-> bool' signature declares that this function MUST return a boolean value.
    // This is the contract the compiler will enforce.

    // The expression `number % 2 == 0` evaluates to a boolean.
    // It checks if the remainder of 'number' divided by 2 is equal to 0.
    // Since this is the last line and has no semicolon, its result is implicitly returned.
    number % 2 == 0
}

fn has_chars(text: &str, first_char: char, second_char: char) -> (bool, bool) {
    // The return type is a two-element tuple where both elements are booleans.
    let has_a = text.contains(first_char);
    let has_z = text.contains(second_char);

    // Return a tuple containing the two boolean results.
    (has_a, has_z)
}

fn has_chars_with_len(text: &str, first_char: char) -> (bool, usize) {
    // The return type is a two-element tuple where both elements are booleans.
    let has_a = text.contains(first_char);
    let length = text.len();

    // Return a tuple containing the two boolean results.
    (has_a, length)
}



fn mystery(){
    
}

fn sum_range(range: Range<i32>) -> i32{
    let mut sum = 0;

    for r in range {
        sum += r;
    }
    
    sum
}

fn sum_array(arr: [i32;7]) -> i32{
    let mut sum = 0;
    for a in arr {
        sum += a;
    }
    
    sum
}

fn add(first:i32, second: i32) -> i32{
    let result = first + second;
    // println!("result of adding {first} and {second} is {result}");
    return result;
    println!("unreachable code");
}

fn add_v1(first:i32, second: i32) -> i32{
    let result = first + second;
    println!("result of adding {first} and {second} is {result}");
    result
    // println!("unreachable code")
}

fn square(number: i32) -> i32 {
    return number * number;
}

fn cube(number: i32) -> i32{
    return number * number * number;
}

fn cube_v1(number: i32) -> i32{
    number * number * number
}


fn my_function() -> i32{
    return 42;
}

fn my_function_v1() -> i32{
    42
}

fn open_store(neighborhood:&str){
    println!("Opening my mobile pizza store in {neighborhood} for day!"); 
}

fn bake_pizza(number_of_pizzas:i32, topping: &[&str]){
    println!("Baking {number_of_pizzas} pizzas... with {topping:?} toppings"); 
}

fn swim_in_profit(){
    println!("So much $$$, so little time!");
}

fn close_store(){
    println!("Closing the store for the day");
}

