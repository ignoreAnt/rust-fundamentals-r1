fn main() {
    let employee_data = ("Molly", 29, "Marketing");
    println!("Employee name is {}", employee_data.0);
    println!("Employee age is {}", employee_data.1);
    println!("Employee department is {}", employee_data.2);
    println!("Employee {:?}", employee_data);
    println!("Employee {:#?}", employee_data);

    let employee_data = ("Molly", 29, "Marketing", 29);
    let (name, age, department ) = ("Molly", 29, "Marketing");
    println!("name : {name} ; age: {age}; department: {department}");
    
}