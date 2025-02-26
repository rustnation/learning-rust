//! src/algorithms/mod.rs
pub mod bubble_sort;
pub mod cocktail_sort;
pub mod comb_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod quick_sort;
pub mod selection_sort;

pub fn master(show: bool) {
    if show {
        // Quick Sort
        quick_sort::master(false);

        // Bubble Sort
        bubble_sort::master(false);

        // Insertion Sort
        insertion_sort::master(false);

        // Selection Sort
        selection_sort::master(false);

        // Merge Sort
        merge_sort::master(false);

        // Cocktail Sort
        cocktail_sort::master(false);

        // Comb Sort
        comb_sort::master(false);
    }
}
