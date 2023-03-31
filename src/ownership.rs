pub mod string_type;
pub mod string_clone_heap;
pub mod functions;
pub mod return_values_and_scope;
pub mod return_multiple_values;
pub mod references_borrowing;
pub mod mutable_references;

pub fn master(show: bool) {
    if show {
        common::print_title("OWNERSHIP");

        // String Type
        string_type::master();

        // String Clone Heap
        string_clone_heap::master();

        // Moving a Value
        functions::master();

        // Return values and scope
        return_values_and_scope::master();

        // Return multiple values
        return_multiple_values::master();

        // References and Borrowing
        references_borrowing::master();

        // Mutable references
        mutable_references::master();
    }
}
