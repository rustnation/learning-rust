pub mod quick_sort;
pub mod bubble_sort;
pub mod insertion_sort;
pub mod selection_sort;
pub mod merge_sort;

pub fn master(show: bool) {
    if show {
        common::print_title("ALGORITHMS");

        // Quick Sort
        quick_sort::master();

        // Bubble Sort
        bubble_sort::master();

        // Insertion Sort
        insertion_sort::master();

        // Selection Sort
        selection_sort::master();

        // Merge Sort
        merge_sort::master();
    }
}
