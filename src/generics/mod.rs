use crate::print_title;

mod activity;
mod conveyor_belt;
mod definition;
mod demo;
mod generic_enums;
mod generic_functions;
mod generic_structs;
mod generic_structures;
mod vehicle;

pub fn master(show: bool) {
    if show {
        print_title("Generics");

        definition::master(false);

        demo::master(false);

        activity::master(false);

        generic_structures::master(false);

        conveyor_belt::master(false);

        vehicle::master(false);

        generic_functions::master(false);

        generic_enums::master(false);

        generic_structs::master(false);
    }
}
