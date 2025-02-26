//! src/closures/mod.rs
pub mod closures_inside_methods;
pub mod definition;
pub mod demo;
pub mod map_collect;
pub mod map_combinator;
pub mod process_closure_thread;

pub fn master(show: bool) {
    if show {
        println!("\n-- Closures");

        definition::master(false);

        process_closure_thread::master(false);

        demo::master(false);

        map_combinator::master(true);

        closures_inside_methods::master(false);

        map_collect::master(false);
    }
}
