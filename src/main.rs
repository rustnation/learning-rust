pub mod common;
pub mod basics;
pub mod ownership;
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
}

fn local_function() {
    println!("--- Local Function ---");
    println!("Hello from local function!");
}
