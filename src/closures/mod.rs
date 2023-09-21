mod definition;
mod demo;
mod map_combinator;
mod process_closure_thread;

pub fn master(show: bool) {
    if show {
        println!("\n-- Closures");

        definition::master(false);

        process_closure_thread::master(false);

        demo::master(false);

        map_combinator::master(true);
    }
}
