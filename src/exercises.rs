pub mod ex03;
pub mod ex08;

pub fn master(show: bool) {
    if show {
        common::print_title("EXERCISES");

        ex03::master(false);

        ex08::master(false);
    }
}
