mod activity;
mod definition;
mod demo;
mod partition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Iterators");

        definition::master(false);

        demo::master(false);

        activity::master(false);

        partition::master(false);
    }
}
