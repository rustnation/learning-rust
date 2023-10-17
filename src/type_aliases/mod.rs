use crate::print_title;
mod demo;

pub fn master(show: bool) {
    if show {
        print_title("Type Aliases");

        demo::master(false);
    }
}
