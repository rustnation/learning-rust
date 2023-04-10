pub mod definition;
pub mod derives;
pub mod returning_traits_with_dyn;

pub fn master(show: bool) {
    if show {
        common::print_title("TRAITS");

        // Trait Definition
        definition::master();

        // Trait using Derive Attribute
        derives::master();

        // Returning Traits with dyn
        returning_traits_with_dyn::master();
    }
}
