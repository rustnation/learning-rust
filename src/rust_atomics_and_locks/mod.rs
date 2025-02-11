//! src/rust_atomics_and_locks/mod.rs
mod chapter01;

pub fn master(show: bool) {
    if show {
        println!("--- Rust Atomics and Locks Book");

        chapter01::master(false);
    }
}
