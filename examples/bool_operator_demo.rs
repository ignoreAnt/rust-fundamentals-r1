fn main() {
    let purchased_ticket: bool = false;
    let plane_on_time: bool = false;
    // (A && B) && (C && D)
    // A && (B && C) && D
    let making_event = purchased_ticket && plane_on_time && (4 >= 6) && (5 <= 6);
    println!("Purchased Ticket: {}, Plane On Time: {}. ==> Making event? {}",
             purchased_ticket, plane_on_time, making_event);
    
    if making_event {
        println!("Hey Made it to the event");
    } else {
        println!("Oh no! missed the event");
    }


    let user_has_paid_for_subscription: bool = false;
    let user_is_admin: bool = false;
    let user_can_see_premium_experience = user_has_paid_for_subscription || user_is_admin;
    println!("Paid: {}, Admin: {}. ==> Can see premium? {}",
             user_has_paid_for_subscription, user_is_admin, user_can_see_premium_experience);
    
    if user_can_see_premium_experience {
        println!("Yay! i can see premium experience");
    }else {
        println!("No! i can't see see premium experience");
    }
    
    // (A && B) || C
    
}