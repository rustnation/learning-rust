mod quick_sort;
mod bubble_sort;
mod insertion_sort;
mod selection_sort;
mod merge_sort;

pub fn master(show: bool) {
    if show {
        //common::print_title("ALGORITHMS");

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
