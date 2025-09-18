//! src/enums/mod.rs
use crate::print_title;

pub mod cards;
pub mod definition;
pub mod demo;
pub mod enum_multi_array;
pub mod enum_with_data;
pub mod enum_with_match;
pub mod event;
pub mod german_months;
pub mod hold_data;
pub mod matches;
pub mod options;
pub mod payment_method_type;

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

        // Enum holding data
        hold_data::master(false);

        // German Months
        german_months::index(false);

        // Cards Demo
        cards::index(false);

        // Payment Method Type
        payment_method_type::index(false);
    }
}
