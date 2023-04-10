pub mod definition;
pub mod derives;

pub fn master(show: bool) {
    if show {
        common::print_title("TRAITS");

        // Trait Definition
        definition::master();

        // Trait using Derive Attribute
        derives::master();
    }
}
