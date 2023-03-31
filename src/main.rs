pub mod basics;
pub mod ownership;
pub mod structs;
pub mod methods;
pub mod enums;
pub mod random_numbers;
pub mod algorithms;
pub mod collections;
pub mod hashmaps;
pub mod strings;
pub mod exercises;
//use std::{cmp::Ordering, io};

fn main() {
    // Basics
    common::print_title("BASICS");
    basics::basics();

    // Local Function
    local_function();

    // Ownership
    common::print_title("OWNERSHIP");
    ownership::ownership();

    // Structs
    common::print_title("STRUCTS");
    structs::master();

    // Methods
    common::print_title("METHODS");
    methods::master();

    // Enums
    common::print_title("ENUMS");
    enums::master();

    // Random Numbers
    common::print_title("RANDOM NUMBERS");
    random_numbers::master();

    // Algorithms
    common::print_title("ALGORITHMS");
    algorithms::master();

    // Vectors
    common::print_title("COLLECTIONS");
    collections::master();

    // HashMaps
    common::print_title("HASHMAPS");
    hashmaps::master();

    // Strings
    common::print_title("STRINGS");
    strings::master();

    // Exercises
    common::print_title("EXERCISES");
    exercises::master();
}

fn local_function() {
    println!("--- Local Function ---");
    println!("Hello from local function!");
}
