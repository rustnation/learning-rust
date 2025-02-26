//! src/decisions/mod.rs
pub mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Decisions");

        definition::master(true);
    }
}
