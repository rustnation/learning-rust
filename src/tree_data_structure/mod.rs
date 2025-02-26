//! src/tree_data_structure/mod.rs
pub mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Tree Data Structure");

        definition::master(true);
    }
}
