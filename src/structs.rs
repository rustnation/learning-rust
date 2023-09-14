pub mod definition;
pub mod tuple_structs;
pub mod unit_struct;
pub mod human;
pub mod integrate_struct_enum_match;
mod structs_owned;
mod person;

pub fn master(show: bool) {
    if show {
        common::print_title("STRUCTS");

        definition::master(false);

        tuple_structs::master(false);

        unit_struct::master(false);

        human::master(false);

        integrate_struct_enum_match::master(false);

        structs_owned::master(false);

        person::master(false);
    }
}
