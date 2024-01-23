mod concatenation;
mod definition;
mod iterate;
mod string_functions;

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
    }
}
