//! src/joinhandle/mod.rs
mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Join Handle");

        definition::master(true);
    }
}
