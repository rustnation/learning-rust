pub mod definition;

pub fn master(show: bool) {
    if show {
        common::print_title("RANDOM NUMBERS");

        // Random Definition
        definition::master();
    }
}
