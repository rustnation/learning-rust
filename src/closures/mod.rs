mod definition;
mod process_closure_thread;
mod demo;
mod map_combinator;

pub fn master(show: bool) {
    if show {
        println!("\n-- Closures");

        definition::master(false);

        process_closure_thread::master(false);

        demo::master(false);

        map_combinator::master(true);
    }
}
