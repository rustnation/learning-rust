pub mod vector;

pub fn master(show: bool) {
    if show {
        common::print_title("- Chapter 8");

        vector::master();
    }
}
