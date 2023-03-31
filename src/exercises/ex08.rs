pub mod vector;
pub mod pig_latin;

pub fn master(show: bool) {
    if show {
        common::print_title("- Chapter 8");

        vector::master();

        pig_latin::master();
    }
}
