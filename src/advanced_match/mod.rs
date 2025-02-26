//! src/advanced_match/mod.rs
pub mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Advanced Match");

        definition::master(true);
    }
}
