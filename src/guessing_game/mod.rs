//! src/guessing_game/mod.rs
pub mod guessing;

pub fn master(show: bool) {
    if show {
        guessing::master(false);
    }
}
