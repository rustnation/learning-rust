use crate::print_title;

mod arrays;
mod booleans;
mod borrowing;
mod branches;
mod characters;
mod clone;
mod compounds;
mod constants;
mod control_flow;
mod floats;
mod functions;
mod if_let;
mod integers;
mod loops;
mod num_operations;
mod scalars;
mod shadowing;
mod slices;
mod variables;

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
        num_operations::master(false);

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

        // Arrays
        arrays::master(false);

        // Clone
        clone::master(false);

        // Borrowing
        borrowing::master(false);

        // Branches
        branches::master(false);

        // Using if in a let statement
        if_let::master(false);
    }
}
