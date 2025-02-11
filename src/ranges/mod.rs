//! src/ranges/mod.rs
mod demo;

pub fn master(show: bool) {
    if show {
        println!("\n-- Ranges");

        demo::master(true);
    }
}
