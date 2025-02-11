//! src/closures/mod.rs
mod closures_inside_methods;
mod definition;
mod demo;
mod map_collect;
mod map_combinator;
mod process_closure_thread;

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
