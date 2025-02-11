//! src/ownership/returning_ownership.rs
use crate::print_title;

pub fn master(show: bool) {
    if show {
        print_title("Returning Ownership");

        let s1 = String::from("hallo");

        let (s2, len) = calculate_length(s1);

        println!("The length of '{s2}' is {len}")
    }
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
