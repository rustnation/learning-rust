//! src/slices/mod.rs
use crate::print_title;
pub mod activity;

pub fn master(show: bool) {
    if show {
        print_title("Slices");

        activity::slices_activity(false);
    }
}
