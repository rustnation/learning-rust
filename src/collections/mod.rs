//! src/collections/mod.rs
mod vectors;

pub fn master(show: bool) {
    if show {
        //common::print_title("COLLECTIONS");
        println!("\n-- Collections");

        // Vectors
        vectors::master(false);
    }
}
