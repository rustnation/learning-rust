use crate::print_title;
mod definition;

pub fn master(show: bool) {
    if show {
        print_title("Shadowing");

        definition::master(false);
    }
}
