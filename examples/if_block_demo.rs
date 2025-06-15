const VOTING_AGE:i32 = 18; 

fn main() {
    
    let group = [18, 17, 16, 19, 20, 19, 23, 24];

    let mut eligible_person_count = 0;
    let mut ineligible_person_count = 0;
    for age in group {
        println!("current element : {}", age);
        if is_eligible_to_vote(age){
            println!("processed eligible_person_count : {}", age);
            eligible_person_count = eligible_person_count + 1;
        }else {
            println!("processed ineligible_person_count : {}", age); 
            ineligible_person_count = ineligible_person_count + 1;
        }
        println!("finished processing element : {}", age);
    }
    
    println!("eligible_person_count : {} ; ineligible_person_count : {}",
             eligible_person_count, ineligible_person_count);
}

fn is_eligible_to_vote(age: i32) -> bool{
    age >= 18
}