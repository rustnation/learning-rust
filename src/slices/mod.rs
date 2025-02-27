//! src/slices/mod.rs
use crate::print_title;
pub mod activity;
pub mod array_slices;
pub mod definition;

pub fn master(show: bool) {
    if show {
        print_title("Slices");

        activity::slices_activity(false);
        definition::master(false);
        array_slices::master(false);
    }
}
