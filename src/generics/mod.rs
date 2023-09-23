use crate::print_title;

mod activity;
mod definition;
mod demo;
mod generic_structures;

pub fn master(show: bool) {
    if show {
        print_title("Generics");

        definition::master(false);

        demo::master(false);

        activity::master(false);

        generic_structures::master(false);
    }
}
