pub mod definition;
pub mod options;
pub mod matches;
pub mod enum_multi_array;
pub mod enum_with_match;

pub fn master(show: bool) {
    if show {
        common::print_title("ENUMS");

        // Enum Definition
        definition::master();

        // Option Enums
        options::master();

        // Matches
        matches::master();

        // Enum Multi Array
        enum_multi_array::master();

        // Enum with Match
        enum_with_match::master();
    }
}
