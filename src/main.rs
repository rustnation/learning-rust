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
pub mod errors;
pub mod traits;
pub mod lifetimes;

//use std::{cmp::Ordering, io};

fn main() {
    // Basics
    basics::master(false);

    // Local Function
    local_function(false);

    // Ownership
    ownership::master(false);

    // Structs
    structs::master(false);

    // Methods
    methods::master(false);

    // Enums
    enums::master(false);

    // Random Numbers
    random_numbers::master(false);

    // Algorithms
    algorithms::master(false);

    // Vectors
    collections::master(false);

    // HashMaps
    hashmaps::master(false);

    // Strings
    strings::master(false);

    // Exercises
    exercises::master(false);

    // Errors
    errors::master(false);

    // Traits
    traits::master(false);

    // Lifetimes
    lifetimes::master(false);

    // Generics
    common::generics::definition::master(false);

    // Tests
    common::tests::definition::master(false);

    // Closures
    common::closures::definition::master(true);
}

fn local_function(show: bool) {
    if show {
        println!("--- Local Function ---");
        println!("Hello from local function!");
    }
}
