//! src/rust_atomics_and_locks/chapter01/mod.rs
pub mod thread_basics;
pub mod thread_closure;
pub mod thread_join;
pub mod thread_leaking;
pub mod thread_result;
pub mod thread_scoped;

pub fn master(show: bool) {
    if show {
        println!("---Chapter 01");

        thread_basics::master(false);
        thread_join::master(false);
        thread_closure::master(false);
        thread_result::master(false);
        thread_scoped::master(false);
        thread_leaking::master(true);
    }
}
