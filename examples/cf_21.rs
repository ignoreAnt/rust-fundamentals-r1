fn combine(t1: (i32, i32), t2: (i32, i32)) -> (i32, i32) {
    (t1.0 + t2.0, t1.1 + t2.1)
}

fn main() {
    let tuple1 = (3, 7);
    let tuple2 = (5, 2);

    let result = combine(tuple1, tuple2);
    println!("Combined tuple: {:?}", result);
}
