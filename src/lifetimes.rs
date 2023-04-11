pub mod definition;

pub fn master(show: bool) {
    if show {
        common::print_title("--- Lifetimes Definition ---");
        definition::master();
    }
}
