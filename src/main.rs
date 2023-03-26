pub mod common;
pub mod basics;
pub mod ownership;
pub mod structs;
pub mod methods;
pub mod enums;
pub mod random_numbers;
//use self::ownership::string_type;
use common::messages;

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
}

fn local_function() {
    println!("--- Local Function ---");
    println!("Hello from local function!");
}
