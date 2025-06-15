fn main() {
    let some_bool = false;
    // This code would NOT compile:
    match some_bool {
         true => { 
             println!("It was true!"); 
         }
         false => {
             println!("It was false!"); 
         } 
        // Compiler Error: missing match arm: `false` is not covered
    }
    
    // if some_bool {
    //     println!("It was true!");
    // }
    // Symbols
    // -> (arrow symbol ) what kind of values can be returned from a function
    // => (rocket symbol) matching the values in match construct in each arm
    
    let value = match some_bool {
        true => {20}
        false => {40}
    };

    println!("{value}");
}