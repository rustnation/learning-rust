//! src/new_type/mod.rs
use crate::print_title;
pub mod activity;
pub mod definition;
pub mod demo;

pub fn master(show: bool) {
    if show {
        print_title("New Type Pattern");

        demo::master(false);

        activity::master(false);

        definition::master(false);
    }
}
