//! src/structs/mod.rs
pub mod definition;
pub mod file;
pub mod human;
pub mod integrate_struct_enum_match;
pub mod person;
pub mod queue;
pub mod struct_update_syntax;
pub mod structs_owned;
pub mod tuple_structs;
pub mod unit_struct;

pub fn master(show: bool) {
    if show {
        definition::master(false);

        tuple_structs::master(false);

        unit_struct::master(false);

        human::master(false);

        integrate_struct_enum_match::master(false);

        structs_owned::master(false);

        person::master(false);

        queue::master(false);

        file::master(false);

        struct_update_syntax::master(false);
    }
}
