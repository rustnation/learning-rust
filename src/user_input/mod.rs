//! src/user_input/mod.rs
mod activity;
mod demo;

pub fn master(show: bool) {
    if show {
        println!("\n--- User Input");

        demo::master(false);
        activity::master(true);
    }
}
