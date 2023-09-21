mod bubble_sort;
mod insertion_sort;
mod merge_sort;
mod quick_sort;
mod selection_sort;

pub fn master(show: bool) {
    if show {
        // Quick Sort
        quick_sort::master(false);

        // Bubble Sort
        bubble_sort::master(false);

        // Insertion Sort
        insertion_sort::master(true);

        // Selection Sort
        selection_sort::master(false);

        // Merge Sort
        merge_sort::master(false);
    }
}
