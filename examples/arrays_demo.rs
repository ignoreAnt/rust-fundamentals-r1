fn main() {
    let student_1_height = 150;
    let student_2_height = 154;
    let student_3_height = 156;
    let student_4_height = 157;
    let student_5_height = 160;
    let student_6_height = 170;
    let student_7_height = 171;
    
    // average = total_height/total_number
    let total_height = student_1_height + student_2_height + student_3_height;
    
    let nums = [2, 5, 6, 8, 9, 10];
    let height_of_students = [150, 154, 156, 157, 160, 170, 171, 178, 173];
    println!("Total number of students are {}", height_of_students.len());
    let mut sum = 0;
    for height in height_of_students{
        // sum = sum + height;
        sum += height;
    }

    println!("average height : {}" , sum/height_of_students.len());
    let index = 5;
    let index_plus_one = index + 1;
    println!("first number is nums is {}", nums[index]);
    // length --> len()
    let apple_varieties: [&str; 3] = ["Granny Smith", "McIntosh", "Red Delicious"];
    println!("Total number of apple varieties are : {}", apple_varieties.len());
    
    let empty_class:[i32; 0] = [];
}