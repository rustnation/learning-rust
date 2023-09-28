use crate::print_title;

mod closures;
mod error_propagation;
mod demo_custom_error;

pub fn master(show: bool) {
    if show {
        print_title("Errors");

        closures::master(false);

        error_propagation::master(false);

        demo_custom_error::master(false);
    }
}
