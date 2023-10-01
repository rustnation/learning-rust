use crate::print_title;

mod closures;
mod demo_custom_error;
mod error_propagation;

pub fn master(show: bool) {
    if show {
        print_title("Errors");

        closures::master(false);

        error_propagation::master(false);

        demo_custom_error::master(false);
    }
}
