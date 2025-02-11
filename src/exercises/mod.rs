//! src/exercises/mod.rs
mod ex03;
mod ex08;

pub fn master(show: bool) {
    if show {
        println!("\n-- Exercises");

        ex03::master(false);

        ex08::master(false);
    }
}
