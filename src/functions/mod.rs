//! src/functions/mod.rs
mod blocks_in_functions;
mod definition;

pub fn master(show: bool) {
    if show {
        println!("-- Functions --");

        definition::master(false);
        blocks_in_functions::master(false);
    }
}
