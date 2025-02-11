//! src/variables_and_mutability/mod.rs
mod constants;
mod variables;

use crate::print_title;

pub fn master(show: bool) {
    if show {
        print_title("Variables and Mutability");

        variables::master(false);
        constants::master(false);
    }
}
