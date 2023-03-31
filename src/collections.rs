pub mod vectors;

pub fn master(show: bool) {
    if show {
        common::print_title("COLLECTIONS");

        // Vectors
        vectors::master();
    }
}