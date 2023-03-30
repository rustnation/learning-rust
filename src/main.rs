pub mod common;
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
use common::messages;
//use std::{cmp::Ordering, io};

fn main() {
    // Basics
    messages::print_title("BASICS");
    basics::basics();

    // Local Function
    local_function();

    // Ownership
    messages::print_title("OWNERSHIP");
    ownership::ownership();

    // Structs
    messages::print_title("STRUCTS");
    structs::master();

    // Methods
    messages::print_title("METHODS");
    methods::master();

    // Enums
    messages::print_title("ENUMS");
    enums::master();

    // Random Numbers
    messages::print_title("RANDOM NUMBERS");
    random_numbers::master();

    // Algorithms
    messages::print_title("ALGORITHMS");
    algorithms::master();

    // Vectors
    messages::print_title("COLLECTIONS");
    collections::master();

    // HashMaps
    messages::print_title("HASHMAPS");
    hashmaps::master();

    // Strings
    messages::print_title("STRINGS");
    strings::master();

    // Exercises
    messages::print_title("EXERCISES");
    exercises::master();
}

fn local_function() {
    println!("--- Local Function ---");
    println!("Hello from local function!");
}
