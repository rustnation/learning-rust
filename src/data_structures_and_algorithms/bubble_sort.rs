use crate::print_title;

pub fn master(show: bool) {
    if show {
        print_title("Bubble Sort");

        let mut nums = vec![5, 3, 1, 4, 2, 7, 6];
        println!("Before sorting: {:?}", nums);

        bubble_sort(&mut nums);
        println!("After sorting: {:?}", nums);
    }
}

fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    let mut swapped;

    // Loop until no more swaps are needed.
    loop {
        swapped = false;

        // Iterate through the array, comparing adjacent elements.
        for i in 0..len - 1 {
            // Swap elements if they are out of order.
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        // If no swaps were made during the iteration, the array is sorted.
        if !swapped {
            break;
        }
    }
}
