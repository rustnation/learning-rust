pub mod quick_sort;
pub mod bubble_sort;
pub mod insertion_sort;
pub mod selection_sort;

pub fn master() {
    // Quick Sort
    quick_sort::master();

    // Bubble Sort
    bubble_sort::master();

    // Insertion Sort
    insertion_sort::master();

    // Selection Sort
    selection_sort::master();
}
