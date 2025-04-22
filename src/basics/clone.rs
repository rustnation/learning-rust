//! src/basics/clone.rs
use crate::print_title;

pub fn index(show: bool) {
    if show {
        print_title("Clone");

        let s1 = String::from("hallo");
        let s2 = s1.clone();

        println!("s1 = {s1}, s2 = {s2}");
    }
}
