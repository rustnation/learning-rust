pub mod basics;
use self::basics::variables;
use self::basics::constants;
use self::basics::shadowing;
use self::basics::scalar_types;
use self::basics::floating_point;
use self::basics::numeric_operations;
use self::basics::boolean_type;
use self::basics::character_type;
use self::basics::compound_types;
use self::basics::functions;
use self::basics::control_flow;
use self::basics::loops;

fn main() {
    // Variables
    variables::variables();

    // Constants
    constants::constants();

    // Shadowing
    shadowing::shadowing();

    // Scalar Types
    scalar_types::scalar_types();

    // Floating Points
    floating_point::floating_points();

    // Numeric Operations
    numeric_operations::numeric_operations();

    // Boolean Types
    boolean_type::boolean_type();

    // Character Types
    character_type::character_type();

    // Tuple Types
    compound_types::tuple_type();

    // Array Type
    compound_types::array_type();

    // Local Function
    local_function();

    // Functions with Parameters
    functions::function_with_parameters(7);

    // Functions Multiple Parameters
    functions::function_multiple_parameters(7, 'h');

    // Function with Return Value
    let seven = functions::function_with_return_value();
    println!("The value of seven is: {seven}");

    // If Expression
    control_flow::if_expression();

    // If with Multiple Expressions
    control_flow::if_multiple_expressions();

    // Returning Values with Loops
    loops::returning_values_from_loops();

    // Loop Labels Multiple Loops
    loops::loop_labels_multiple_loops();

    // Conditional Loops with While
    loops::conditional_loops_with_while();

    // Looping Through a Collection with For
    loops::looping_through_collection_with_for();

    // For Loop Elements
    loops::for_loop_elements();
}

fn local_function() {
    println!("--- Local Function ---");
    println!("Hello from local function!");
}