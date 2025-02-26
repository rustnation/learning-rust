//! src/typestate_pattern/mod.rs
use crate::print_title;

pub mod activity;
pub mod demo;

pub fn master(show: bool) {
    if show {
        print_title("TypeState Pattern");

        demo::master(false);

        activity::master(false);
    }
}
