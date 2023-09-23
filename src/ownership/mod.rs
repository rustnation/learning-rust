mod borrowing;
mod functions;
mod mutable_borrowing;
mod mutable_references;
mod references_borrowing;
mod return_multiple_values;
mod return_values_and_scope;
mod string_clone_heap;
mod string_type;

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
    }
}
