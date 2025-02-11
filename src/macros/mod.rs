//! src/macros/mod.rs
mod create_vector;
mod definition;

pub fn master(show: bool) {
    if show {
        println!("--- Macros");

        definition::master(false);
        create_vector::master(false);
    }
}
