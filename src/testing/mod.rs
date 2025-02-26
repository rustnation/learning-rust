//! src/testing/mod.rs
pub mod activity;
pub mod definition;
pub mod demo;

pub fn master(show: bool) {
    if show {
        println!("\n-- Tests");

        definition::master(false);
        demo::master(false);
        activity::master(true);
    }
}
