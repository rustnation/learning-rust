//! src/macros/mod.rs
pub mod create_vector;
pub mod definition;

pub fn master(show: bool) {
    if show {
        println!("--- Macros");

        definition::master(false);
        create_vector::master(false);
    }
}
