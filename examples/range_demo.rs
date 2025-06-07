use std::ops::Range;

fn main() {
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let exclusive_range:Range<i32> = 1..31; // all the number from 1 to 30
    let inclusive_range = 1..=31; // 1 to 30
    println!("exclusive_range :{:?}", exclusive_range);
    println!("inclusive_range :{:#?}", inclusive_range);
    
    // for loop
    for r in array{
        print!("{} ", r + 4);  
    }

    println!("\nPrinting the ranges");
    for ra in exclusive_range {
        // print!("{:.2} ", ra as f64 * 0.6);
        print!("{:.2} ", ra);
    }

    println!("\nPrinting the ranges");
    for ra in inclusive_range {
        print!("{:.2} ", ra);
    }
    println!("\nPrinting the char ranges");
    let chars = 'b' ..='p';
    for ch in chars {
        print!("{}", ch);
    }

    let colors = ["red", "green", "yellow"];

    println!("\nFavorite colors:");
    for color_item in colors { // A common convention: singular 'color_item' for plural 'colors'
        println!("{} is a great color!", color_item);
    }
    
    let decimal_range = 1.0..=31.0;
    
    if 30 < 45{
        
    } else { 
        
    }
    
}