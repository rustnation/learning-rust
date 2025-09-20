//! src/functions/mod.rs
pub mod blocks_in_functions;
pub mod definition;
pub mod func_return_option;

pub fn master(show: bool) {
    if show {
        println!("-- Functions --");

        definition::master(false);
        blocks_in_functions::master(false);
        func_return_option::index(false);
    }
}
