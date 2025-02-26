//! src/ownership/mod.rs
pub mod borrowing;
pub mod dereference_operator;
pub mod functions;
pub mod mutable_borrowing;
pub mod mutable_references;
pub mod ownership_arrays;
pub mod references_borrowing;
pub mod return_multiple_values;
pub mod return_values_and_scope;
pub mod returning_ownership;
pub mod string_clone_heap;
pub mod string_type;

pub fn master(show: bool) {
    if show {
        // String Type
        string_type::master(false);

        // String Clone Heap
        string_clone_heap::master(false);

        // Moving a Value
        functions::master(false);

        // Return values and scope
        return_values_and_scope::master(false);

        // Return multiple values
        return_multiple_values::master(false);

        // References and Borrowing
        references_borrowing::master(false);

        // Mutable references
        mutable_references::master(false);

        // Borrowing
        borrowing::master(false);

        // Mutable Borrowing of Variables
        mutable_borrowing::master(false);

        // Returning Ownership
        returning_ownership::master(false);

        // Dereference Operator
        dereference_operator::master(false);

        // Ownership in Arrays
        ownership_arrays::master(false);
    }
}
