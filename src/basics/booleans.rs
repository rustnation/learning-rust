//! src/basics/booleans.rs
pub fn master(show: bool) {
    if show {
        println!("--- Boolean Type ---");

        let t = true;
        println!("value of t is: {t}");

        let f = false;
        println!("value of f is: {f}");

        let age = 50;
        if age <= 20 {
            println!("the person is young");
        } else if age > 20 || age <= 35 {
            println!("the person is in the middle age");
        } else if age > 35 || age <= 50 {
            println!("the person is mature");
        } else if age > 50 {
            println!("the person is getting old");
        }

        demo_and_logic(false);
        demo_or_logic(false);
    }
}

pub fn demo_and_logic(show: bool) {
    if show {
        let purchased_ticket = true;
        let plane_on_time = true;
        let making_event = purchased_ticket && plane_on_time;
        println!("It is {} that I will arrive as expected", making_event);
    }
}

pub fn demo_or_logic(show: bool) {
    if show {
        let user_has_paid_for_subscription = true;
        let user_is_admin = true;
        let is_user_granted = user_has_paid_for_subscription || user_is_admin;
        println!(
            "Is user allowed to access the premium course?: {}",
            is_user_granted
        );
    }
}
