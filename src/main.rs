fn main() {
    // Hello World
    common::hello_world::master(false);

    // Guessing Game
    common::guessing_game::master(false);

    // Basics
    common::basics::master(false);

    // Local Function
    local_function(false);

    // Ownership
    common::ownership::master(false);

    // Structs
    common::structs::master(false);

    // Methods
    common::methods::master(false);

    // Enums
    common::enums::master(false);

    // Random Numbers
    common::random_numbers::master(false);

    // Algorithms
    common::algorithms::master(false);

    // Vectors
    common::collections::master(false);

    // HashMaps
    common::hashmaps::master(false);

    // Exercises
    common::exercises::master(false);

    // Errors
    common::errors::master(false);

    // Traits
    common::traits::master(false);

    // Lifetimes
    common::lifetimes::master(false);

    // Strings
    common::strings::master(false);

    // Generics
    common::generics::master(false);

    // Tests
    common::testing::master(false);

    // Closures
    common::closures::master(false);

    // Iterators
    common::iterators::master(false);

    // Doc
    common::doc::master(false);

    // Boxes
    common::boxes::master(false);

    // Custom Smart Pointer
    common::custom_smart_pointer::master(false);

    // Rc<T>
    common::rc_t::master(false);

    // Messenger Application
    common::messenger::master(false);

    // Tree Data Structure
    common::tree_data_structure::master(false);

    // Concurrency
    common::threads::master(false);

    // JoinHandle
    common::joinhandle::master(false);

    // Message Passing
    common::message_passing::master(false);

    // The API of Mutex<T>
    common::api_mutex::master(false);

    // Object Oriented
    common::object_oriented::master(false);

    // Draw Trait
    common::draw::master(false);

    // Blog Post
    common::post::master(false);

    // Patterns
    common::patterns::master(false);

    // Destructuring Structs
    common::destructuring_structs::master(false);

    // Unsafe Code
    common::unsafe_code::master(false);

    // Using Extern Functions
    common::extern_functions::master(false);

    // Accessing or Modifying a Mutable Static Variable
    common::static_variable::master(false);

    // Implementing an Unsafe Trait
    common::unsafe_trait::master(false);

    // Operator Overloading
    common::operator_overloading::master(false);

    // Implementing the Add trait on Millimeters to add Millimeters and Meters
    common::add_millimeters_to_meters::master(false);

    // Vectors
    common::vectors::master(false);

    // Futures
    common::futures::master(false);

    // Get Type
    common::get_type::master(false);

    // Mutable Reference
    common::mutable_reference::master(false);

    // Making Decisions with Rust
    common::decisions::master(false);

    // Match Expression
    common::matches::master(false);

    // Loops
    common::loops::master(false);

    // While Loops
    common::while_loop::master(false);

    // Tuples
    common::tuples::master(false);

    // Expressions
    common::expressions::master(false);

    // Arrays
    common::arrays::master(false);

    // Print variable type
    common::print_var_type::master(false);

    // Move vars
    common::move_vars::master(false);

    // Data Collections
    common::data_collections::master(false);

    // References
    common::references::master(false);

    // Advanced Match
    common::advanced_match::master(false);

    // Optionals
    common::optionals::master(false);

    // Results
    common::results::master(false);

    // Ranges
    common::ranges::master(false);

    // Modules
    common::modules::master(false);

    // External Crates
    common::external_crates::master(false);

    // User Input
    common::user_input::master(false);

    // Webserver
    common::webserver::master(false);

    // New Type Pattern
    common::new_type::master(false);

    // TypeState Pattern
    common::typestate_pattern::master(false);

    // For Loops
    common::for_loops::master(false);

    // Slices
    common::slices::master(false);

    // Type Aliases
    common::type_aliases::master(false);

    // Data Structures and Algorithms
    common::data_structures_and_algorithms::master(false);
}

fn local_function(show: bool) {
    if show {
        println!("--- Local Function ---");
        println!("Hello from local function!");
    }
}
