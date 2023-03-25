pub mod definition;
pub mod tuple_structs;
pub mod unit_struct;

pub fn master() {
    definition::master();

    tuple_structs::master();

    unit_struct::master();
}
