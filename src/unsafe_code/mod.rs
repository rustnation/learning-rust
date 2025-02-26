//! src/unsafe_code/mod.rs
pub mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Unsafe Code");

        definition::master(true);
    }
}
