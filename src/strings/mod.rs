mod concatenation;
mod definition;
mod iterate;
mod string_functions;
mod string_is_encoded;

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
    }
}
