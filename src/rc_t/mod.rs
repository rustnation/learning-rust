//! src/rc_t/mod.rs
pub mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- RC T");

        definition::master(true);
    }
}
