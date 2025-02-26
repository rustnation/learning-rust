//! src/type_aliases/mod.rs
use crate::print_title;
pub mod demo;
pub mod long_types;

pub fn master(show: bool) {
    if show {
        print_title("Type Aliases");

        demo::master(false);

        long_types::master(false);
    }
}
