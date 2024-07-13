use crate::print_title;

mod activity;
mod compare_activity;
mod definition;
mod demo;
mod example;
mod named_lifetime;

pub fn master(show: bool) {
    if show {
        print_title("Lifetimes");

        definition::master(false);

        example::master(false);

        demo::master(false);

        activity::master(false);

        compare_activity::master(false);

        named_lifetime::master(false);
    }
}
