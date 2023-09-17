mod definition;
mod activity;

pub fn master(show: bool) {
    if show {
        common::print_title("HASHMAPS");

        // HashMap Definition
        definition::master(false);

        // HashMap Activity
        activity::master(true);
    }
}
