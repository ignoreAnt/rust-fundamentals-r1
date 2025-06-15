fn main() {
    // 1. login
    // case 1: username + password provided is correct 
    // case 2: username is correct but password is wrong
    // case 3: username is wrong but password is correct
    
    let is_username_correct = true;
    let is_password_correct = true;
    let is_login_successful = login(is_username_correct, is_password_correct);
    println!("is_login_successful : {is_login_successful}");
    
    // 2. Library
    // If you borrow the book
    // case 1: return it back before time => not fine
    // case 2: return it after the borrow time ==> fine
    // case 3: don't return it ==> fine
    // case 4: return it back before time but with some pages torn => fine
    let is_book_returned_before_time = true;
    let is_book_intact = false;
    library_book_handling(is_book_returned_before_time, is_book_intact);
    
}

fn library_book_handling(is_book_returned_before_time: bool,is_book_intact: bool){
    if is_book_returned_before_time && is_book_intact {
        println!("Thank you for using the library and come bak again");
    }else {
        let fine_amount = fine_amount(is_book_returned_before_time, is_book_intact);
        println!("Sorry you need to pay fine : {fine_amount}");

    }
}

fn fine_amount(is_book_returned_before_time: bool, is_book_intact: bool) -> f64{
    
    if !is_book_returned_before_time && is_book_intact{  // case : book is returned before time but not intact
        return 10.0
    }else if !is_book_returned_before_time { 
        return 100.0
    }else if is_book_returned_before_time && !is_book_intact{ 
        return 15.0
    }else { 
        return 5.0
    }
    
    0.0
}

fn login(is_username_correct: bool, is_password_correct: bool) -> bool{
    
    if is_username_correct && is_password_correct {
        println!("You are successfully logged in");
        true
    }else{
        println!("Your username or password is incorrect");
        false
    }
    
}