fn are_arrays_equal(arr1: [&str; 2], arr2: [&str; 2]) -> bool {
    arr1 == arr2
}

fn main() {
    let array1 = ["apple", "banana"];
    let array2 = ["apple", "banana"];
    let array3 = ["orange", "banana"];

    println!("array1 == array2? {}", are_arrays_equal(array1, array2)); // true
    println!("array1 == array3? {}", are_arrays_equal(array1, array3)); // false
}
