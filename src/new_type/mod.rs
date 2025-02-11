//! src/new_type/mod.rs
use crate::print_title;
mod activity;
mod definition;
mod demo;

pub fn master(show: bool) {
    if show {
        print_title("New Type Pattern");

        demo::master(false);

        activity::master(false);

        definition::master(false);
    }
}
