//! src/guessing_game/mod.rs
use crate::print_title;
pub mod guessing;

pub fn master(show: bool) {
    if show {
        guessing::master(false);
    }
}
