//! src/user_input/mod.rs
pub mod activity;
pub mod demo;

pub fn master(show: bool) {
    if show {
        println!("\n--- User Input");

        demo::master(false);
        activity::master(true);
    }
}
