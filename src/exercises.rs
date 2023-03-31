pub mod ex03;

pub fn master(show: bool) {
    if show {
        common::print_title("EXERCISES");

        ex03::master(false);
    }
}
