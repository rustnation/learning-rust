use crate::print_title;
use std::collections::HashSet;

pub fn master(show: bool) {
    if show {
        print_title("Partition");

        in_array(false);
        in_string(false);
    }
}

fn in_array(show: bool) {
    if show {
        let squares = [4, 9, 16, 25, 36, 49, 64];
        println!("squares: {:?}", squares);

        let (powers_of_two, impure): (HashSet<i32>, HashSet<i32>) =
            squares.iter().partition(|&n| n & (n - 1) == 0);

        println!("powers_of_two: {:?}", powers_of_two);
        println!("impure: {:?}", impure);
    }
}

fn in_string(show: bool) {
    if show {
        let (upper, lower): (String, String) = "Great Teacher Onisuka"
            .chars()
            .partition(|&c| c.is_uppercase());

        println!("upper: {:?}", upper);
        println!("lower: {:?}", lower);
    }
}
