//! src/custom_smart_pointer/mod.rs
mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Custom Smart Pointers");

        definition::master(true);
    }
}
