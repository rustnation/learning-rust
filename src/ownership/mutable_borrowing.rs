//! src/ownership/mutable_borrowing.rs
pub fn index(show: bool) {
    if show {
        let mut meal = String::from("burger");
        change_meal(&mut meal);
        println!("The final meal is: {}", meal);
    }
}

fn change_meal(value: &mut String) {
    value.push_str(" and fries");
}
