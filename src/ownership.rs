pub mod string_type;
pub mod string_clone_heap;
pub mod functions;

pub fn ownership() {
    // String Type
    string_type::string_type();

    // String Clone Heap
    string_clone_heap::string_clone_heap();

    // Moving a Value
    functions::moving_value();
}
