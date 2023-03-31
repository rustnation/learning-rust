pub mod definition;
pub mod tuple_structs;
pub mod unit_struct;

pub fn master(show: bool) {
    if show {
        common::print_title("STRUCTS");

        definition::master();

        tuple_structs::master();

        unit_struct::master();
    }
}
