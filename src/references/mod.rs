//! src/references/mod.rs
mod basic_reference;
mod definition;
mod mut_reference;
mod ref_of_references;

pub fn master(show: bool) {
    if show {
        println!("\n-- References");

        definition::master(false);

        ref_of_references::master(false);

        basic_reference::master(false);

        mut_reference::master(false);
    }
}
