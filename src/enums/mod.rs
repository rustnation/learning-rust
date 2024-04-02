use crate::print_title;

mod definition;
mod demo;
mod enum_multi_array;
mod enum_with_data;
mod enum_with_match;
mod event;
mod matches;
mod options;

pub fn master(show: bool) {
    if show {
        print_title("Enums");

        // Enum Demo
        demo::master(false);

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

        // Enum with Data
        enum_with_data::master(false);

        // Event enum
        event::master(false);
    }
}
