pub mod definition;
pub mod options;
pub mod matches;

pub fn master(show: bool) {
    if show {
        common::print_title("ENUMS");

        // Enum Definition
        definition::master();

        // Option Enums
        options::master();

        // Matches
        matches::master();
    }
}
