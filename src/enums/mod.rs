mod demo;
mod definition;
mod options;
mod matches;
mod enum_multi_array;
mod enum_with_match;

pub fn master(show: bool) {
    if show {
        println!("\n-- Enums");

        // Enum Demo
        demo::master(true);

        // Enum Definition
        definition::master(false);

        // Option Enums
        options::master(false);

        // Matches
        matches::master(false);

        // Enum Multi Array
        enum_multi_array::master(false);

        // Enum with Match
        enum_with_match::master(false);
    }
}
