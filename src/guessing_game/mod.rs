//! src/guessing_game/mod.rs
use crate::print_title;
mod guessing;

pub fn master(show: bool) {
    if show {
        print_title("Guessing Game");

        guessing::master(false);
    }
}
