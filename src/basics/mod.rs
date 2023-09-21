pub mod boolean_type;
pub mod character_type;
pub mod compound_types;
pub mod constants;
pub mod control_flow;
pub mod floating_point;
pub mod functions;
pub mod integers;
pub mod loops;
pub mod numeric_operations;
pub mod scalar_types;
pub mod shadowing;
pub mod slices;
pub mod variables;

pub fn master(show: bool) {
    if show {
        //common::print_title("BASICS");

        // Variables
        variables::master();

        // Constants
        constants::master();

        // Shadowing
        shadowing::master();

        // Scalar Types
        scalar_types::master();

        // Floating Points
        floating_point::master();

        // Numeric Operations
        numeric_operations::master();

        // Boolean Types
        boolean_type::master();

        // Character Types
        character_type::master();

        // Tuple Types
        compound_types::tuple_type();

        // Array Type
        compound_types::array_type();

        // Functions with Parameters
        functions::function_with_parameters(7);

        // Functions Multiple Parameters
        functions::function_multiple_parameters(7, 'h');

        // sum 2 numbers
        let result = functions::sum(3, 4);

        // display the result
        functions::display_result(result);

        // Function with Return Value
        let seven = functions::function_with_return_value();
        println!("The value of seven is: {seven}");

        // If Expression
        control_flow::if_expression();

        // If with Multiple Expressions
        control_flow::if_multiple_expressions();

        // For Loop Elements
        loops::for_loop_elements();

        // Countdown Loop with Rev
        loops::countdown_loop_with_rev();

        // Slices
        slices::master();

        // Integers
        integers::master();
    }
}
