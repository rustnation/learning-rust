mod definition;
mod options;
mod matches;
mod enum_multi_array;
mod enum_with_match;

pub fn master(show: bool) {
    if show {
        println!("\n-- Enums");

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
