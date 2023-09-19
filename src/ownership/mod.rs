mod string_type;
mod string_clone_heap;
mod functions;
mod return_values_and_scope;
mod return_multiple_values;
mod references_borrowing;
mod mutable_references;
mod borrowing;
mod mutable_borrowing;

pub fn master(show: bool) {
    if show {
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

        // Borrowing
        borrowing::master();

        // Mutable Borrowing of Variables
        mutable_borrowing::master();
    }
}
