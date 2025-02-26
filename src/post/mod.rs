//! src/post/mod.rs
pub mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Post");

        definition::master(true);
    }
}
