//! src/exercises/mod.rs
pub mod ex03;
pub mod ex08;

pub fn master(show: bool) {
    if show {
        println!("\n-- Exercises");

        ex03::master(false);

        ex08::master(false);
    }
}
