mod definition;
mod human;
mod integrate_struct_enum_match;
mod person;
mod structs_owned;
mod tuple_structs;
mod unit_struct;
mod queue;

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
    }
}
