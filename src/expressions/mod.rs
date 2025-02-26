//! src/expressions/mod.rs
pub mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Expressions");

        definition::master(true);
    }
}
