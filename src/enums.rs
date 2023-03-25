pub mod definition;
pub mod options;
pub mod matches;

pub fn master() {
    // Enum Definition
    definition::master();

    // Option Enums
    options::master();

    // Matches
    matches::master();
}
