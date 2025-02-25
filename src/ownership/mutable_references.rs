//! src/ownership/mutable_references.rs
pub fn master(show: bool) {
    if show {
        let mut s = String::from("hallo");

        change(&mut s);

        immutable_and_mutable_references();

        let burger = String::from("Burger"); // the burger variable is the owner of the
                                             // String
        add_fries(burger); // burger passes the ownership of the String to meal variable

        let pizza = String::from("pizza");
        show_my_meal(&pizza); // we are borrowing the variable to the show_my_meal function
    }
}

fn change(some_string: &mut String) {
    some_string.push_str(", Welt");
}

fn immutable_and_mutable_references() {
    let mut s = String::from("hallo");

    // r1 and r2 have an immutable reference to s
    // here there is a guarantee that no-one is going to change s
    let r1 = &s; // read-only
    let r2 = &s; // read-only
    println!("values of {r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // not problem
    println!("value of {r3}");
}

// the parameter meal must be mutable because it needs internally in the function to mutate the
// meal variable
fn add_fries(mut meal: String) {
    meal.push_str(" and fries");
    println!("meal: {meal}");
}

fn show_my_meal(meal: &String) {
    println!("Meal steps: {meal}");
}
