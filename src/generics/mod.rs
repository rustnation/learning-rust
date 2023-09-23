use crate::print_title;

mod activity;
mod definition;
mod demo;

pub fn master(show: bool) {
    if show {
        print_title("Generics");

        definition::master(false);

        demo::master(false);

        activity::master(true);
    }
}
