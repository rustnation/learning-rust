use crate::print_title;

mod activity;
mod demo;

pub fn master(show: bool) {
    if show {
        print_title("TypeState Pattern");

        demo::master(false);

        activity::master(false);
    }
}
