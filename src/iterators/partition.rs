use crate::print_title;
use std::collections::HashSet;

pub fn master(show: bool) {
    if show {
        print_title("Partition");

        let squares = [4, 9, 16, 25, 36, 49, 64];
        println!("squares: {:?}", squares);

        let (powers_of_two, impure): (HashSet<i32>, HashSet<i32>) =
            squares.iter().partition(|&n| n & (n - 1) == 0);

        println!("powers_of_two: {:?}", powers_of_two);
        println!("impure: {:?}", impure);
    }
}
