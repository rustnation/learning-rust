mod definition;
mod demo;
mod enum_multi_array;
mod enum_with_match;
mod matches;
mod options;

pub fn master(show: bool) {
    if show {
        println!("\n-- Enums");

        // Enum Demo
        demo::master(false);

        // Enum Definition
        definition::master(false);

        // Option Enums
        options::master(false);

        // Matches
        matches::master(true);

        // Enum Multi Array
        enum_multi_array::master(false);

        // Enum with Match
        enum_with_match::master(false);
    }
}
