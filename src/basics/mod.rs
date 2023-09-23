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
        variables::master(false);

        // Constants
        constants::master(false);

        // Shadowing
        shadowing::master(false);

        // Scalar Types
        scalar_types::master(false);

        // Floating Points
        floating_point::master(false);

        // Numeric Operations
        numeric_operations::master(false);

        // Boolean Types
        boolean_type::master(false);

        // Character Types
        character_type::master(false);

        // Tuple Types
        compound_types::master(false);

        // Functions
        functions::master(false);

        // Control Flow
        control_flow::master(false);

        // Loops
        loops::master(false);

        // Slices
        slices::master(false);

        // Integers
        integers::master(false);
    }
}
