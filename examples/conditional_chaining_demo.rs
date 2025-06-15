fn main() {
    // check_seasons_chaining();
    // check_seasons_without_chaining();

    check_seasons_chaining_v1("summer");
    check_seasons_chaining_v1("winter");
    check_seasons_chaining_v1("fall");
    check_seasons_chaining_v1("spring");
    check_seasons_chaining_v1("autumn");
    check_seasons_chaining_v1("monsoon");
    // println!("{}", is_eligible_for_voting(17, true, true));
    // println!("{}", is_eligible_for_voting(18, false, true));
    // println!("{}", is_eligible_for_voting(18, true, false));
    // println!("{}", is_eligible_for_voting(18, true, true));
    
    let (is_eligible, message) = 
        is_eligible_for_voting(18, true, true);
    if !is_eligible {
        println!("{}", message.to_uppercase());
    }else if is_eligible{ 
        println!("{}",message.to_uppercase() )
    }
}

fn check_seasons_chaining(){
    let season = "monsoon";

    if season == "summer" {
        println!("School's out for summer!");
    } else if season == "winter" {
        println!("Brr, it's so cold!");
    } else if season == "fall" {
        println!("The leaves are falling.");
    } else if season == "spring" {
        println!("Lots of rain, but flowers are blooming.");
    } else {
        println!("None of seasons matched!");
    }

    println!("That is why we like seasons");  
}

fn check_seasons_chaining_v1(season: &str){
    
    match season {
        "summer" => {println!("School's out for summer!");}
        "winter" => println!("Brr, it's so cold!"),
        "fall" => println!("The leaves are falling."),
        "spring" => println!("Lots of rain, but flowers are blooming."),
        _ =>  println!("None of seasons matched!"),
    }
}

fn check_seasons_without_chaining(){
    let season = "monsoon";

    if season == "summer" {
        println!("School's out for summer!");
    } 
    
    if season == "winter" {
        println!("Brr, it's so cold!");
    } 
    
    if season == "fall" {
        println!("The leaves are falling.");
    }
    
    if season == "spring" {
        println!("Lots of rain, but flowers are blooming.");
    }

    println!("That is why we like seasons");
}

fn is_eligible_for_voting(age: i32, is_student: bool, has_college_id: bool) -> (bool, String){
    
    if age < 18 {
      return (false, "Your age is less than the eligible voting age, so you can't vote".to_string())
    }
    
    if !is_student {
        return (false, "You are not in regular student of university, so you can't vote".to_string())
    }
    
    if !has_college_id {
        return (false, "You don't have a valid college id of university, so you can't vote".to_string())
    }

    (true, "You are eligible to vote".to_string())
}

fn is_eligible_for_voting_v1(age: i32, is_student: bool, has_college_id: bool) -> bool{
    age >= 18 && is_student && has_college_id
}