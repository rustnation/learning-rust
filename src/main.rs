pub mod common;
pub mod basics;
pub mod ownership;
pub mod structs;
pub mod methods;
pub mod enums;
//use self::ownership::string_type;

fn main() {
    // Basics
    common::messages::print_title("BASICS");
    basics::basics();

    // Local Function
    local_function();

    // Ownership
    common::messages::print_title("OWNERSHIP");
    ownership::ownership();

    // Structs
    common::messages::print_title("STRUCTS");
    structs::master();

    // Methods
    common::messages::print_title("METHODS");
    methods::master();

    // Enums
    common::messages::print_title("ENUMS");
    enums::master();
}

fn local_function() {
    println!("--- Local Function ---");
    println!("Hello from local function!");
}
