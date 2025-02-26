//! src/strings/mod.rs
pub mod borrowed_string;
pub mod concatenation;
pub mod definition;
pub mod iterate;
pub mod string_functions;
pub mod string_is_encoded;
pub mod string_literals;

pub fn master(show: bool) {
    if show {
        // String Definition
        definition::master(false);

        // String Concatenation
        concatenation::master(false);

        // Methods for Iterating Over Strings
        iterate::master(false);

        // String Functions
        string_functions::master(false);

        // &str and String are encoded with UTF-8
        string_is_encoded::master(false);

        // String Literals
        string_literals::master(false);

        // Borrowed String
        borrowed_string::master(false);
    }
}
