//! src/object_oriented/mod.rs
mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Object Oriented");

        definition::master(true);
    }
}
