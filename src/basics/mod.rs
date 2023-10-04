use crate::print_title;

pub mod booleans;
pub mod characters;
pub mod compounds;
pub mod constants;
pub mod control_flow;
pub mod floats;
pub mod functions;
pub mod integers;
pub mod loops;
pub mod numeric_operations;
pub mod scalars;
pub mod shadowing;
pub mod slices;
pub mod variables;

pub fn master(show: bool) {
    if show {
        print_title("BASICS");

        // Variables
        variables::master(false);

        // Constants
        constants::master(false);

        // Shadowing
        shadowing::master(false);

        // Scalar Types
        scalars::master(false);

        // Floating Points
        floats::master(false);

        // Numeric Operations
        numeric_operations::master(false);

        // Boolean Types
        booleans::master(false);

        // Character Types
        characters::master(false);

        // Tuple Types
        compounds::master(false);

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
