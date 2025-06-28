fn count_to_five() {
    let mut counter = 0;

    loop {
        if counter == 5 {
            println!("Done!");
            break;
        }
        println!("Counter: {}", counter);
        counter += 1;
    }
}

fn main() {
    count_to_five();
}

