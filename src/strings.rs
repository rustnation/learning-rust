pub mod definition;
pub mod concatenation;
pub mod iterate;

pub fn master() {
    // String Definition
    definition::master();

    // String Concatenation
    concatenation::master();

    // Methods for Iterating Over Strings
    iterate::master();
}
