//! src/errors/mod.rs
use crate::print_title;

pub mod activity_custom_errors;
pub mod closures;
pub mod custom_errors;
pub mod demo_custom_error;
pub mod error_handling;
pub mod error_propagation;

pub fn master(show: bool) {
    if show {
        print_title("Errors");

        closures::master(false);

        error_propagation::master(false);

        demo_custom_error::master(false);

        activity_custom_errors::master(false);

        error_handling::master(false);

        custom_errors::master(false);
    }
}
