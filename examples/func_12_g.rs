fn sum_array(arr: [i32; 3]) -> i32 {
    arr[0] + arr[1] + arr[2]
}

fn main() {
    let numbers = [10, 20, 30];
    let total = sum_array(numbers);
    println!("The sum of the array is: {}", total);
}
