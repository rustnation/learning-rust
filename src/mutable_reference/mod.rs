//! src/mutable_reference/mod.rs
pub mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Mutable Reference");

        definition::master(false);
    }
}
