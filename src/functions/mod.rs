//! src/functions/mod.rs
pub mod blocks_in_functions;
pub mod definition;

pub fn master(show: bool) {
    if show {
        println!("-- Functions --");

        definition::master(false);
        blocks_in_functions::master(false);
    }
}
