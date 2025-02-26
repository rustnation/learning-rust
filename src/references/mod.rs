//! src/references/mod.rs
pub mod basic_reference;
pub mod definition;
pub mod mut_reference;
pub mod ref_of_references;

pub fn master(show: bool) {
    if show {
        println!("\n-- References");

        definition::master(false);

        ref_of_references::master(false);

        basic_reference::master(false);

        mut_reference::master(false);
    }
}
