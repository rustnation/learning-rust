use crate::print_title;
mod bubble_sort;

pub fn master(show: bool) {
    if show {
        print_title("Data Structures and Algorithms");

        // bubble sort
        bubble_sort::master(false);
    }
}
