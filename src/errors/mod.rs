use crate::print_title;

mod activity_custom_errors;
mod closures;
mod demo_custom_error;
mod error_propagation;

pub fn master(show: bool) {
    if show {
        print_title("Errors");

        closures::master(false);

        error_propagation::master(false);

        demo_custom_error::master(false);

        activity_custom_errors::master(false);
    }
}
