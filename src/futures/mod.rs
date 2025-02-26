//! src/futures/mod.rs
pub mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Futures");

        definition::master(true);
    }
}
