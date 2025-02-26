//! src/cow_type/mod.rs
use crate::print_title;
pub mod definition;

pub fn master(show: bool) {
    if show {
        print_title("Using Cow");

        definition::master(false);
    }
}
