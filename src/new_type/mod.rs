use crate::print_title;
mod demo;
mod activity;

pub fn master(show: bool) {
    if show {
        print_title("New Type Pattern");

        demo::master(false);

        activity::master(false);
    }
}
