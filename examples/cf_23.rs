fn main() {
    let is_daytime = true;

    let light_level = {
        if is_daytime {
            100
        } else {
            0
        }
    };

    println!("Light level: {}", light_level);
}
