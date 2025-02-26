//! src/external_crates/mod.rs
pub mod activity;
pub mod demo;

pub fn master(show: bool) {
    if show {
        println!("\n-- External Crates");

        demo::master(false);
        activity::master(true);
    }
}
