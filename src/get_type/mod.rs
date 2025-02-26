//! src/get_type/mod.rs
pub mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Get Type");

        definition::master(true);
    }
}
