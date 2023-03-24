pub mod string_type;
pub mod string_clone_heap;
pub mod functions;
pub mod return_values_and_scope;

pub fn ownership() {
    // String Type
    string_type::string_type();

    // String Clone Heap
    string_clone_heap::string_clone_heap();

    // Moving a Value
    functions::moving_value();

    // Return values and scope
    return_values_and_scope::return_values_and_scope();
}
