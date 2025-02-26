//! src/basics/mod.rs
use crate::print_title;

pub mod arrays;
pub mod booleans;
pub mod borrowing;
pub mod branches;
pub mod characters;
pub mod clone;
pub mod compounds;
pub mod constants;
pub mod control_flow;
pub mod convert_values;
pub mod floats;
pub mod functions;
pub mod if_let;
pub mod integers;
pub mod loops;
pub mod match_statements;
pub mod num_operations;
pub mod recursion;
pub mod scalars;
pub mod scopes;
pub mod shadowing;
pub mod slices;
pub mod underscores;
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
        slices::master(true);

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

        // Using underscores in numbers
        underscores::master(false);

        // Convert values from one to another
        convert_values::convert_val(false);

        // Match Statement
        match_statements::master(false);

        // Recursion
        recursion::master(false);

        // Scopes
        scopes::master(true);
    }
}
