//! src/boxes/mod.rs
pub mod definition;
pub mod dereference;

pub fn master(show: bool) {
    if show {
        println!("\n-- Boxes");

        definition::master(true);
    }
}
