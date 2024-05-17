mod activity;
mod definition;
mod demo;
mod extend;
mod iter_in_collections;
mod partition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Iterators");

        definition::master(false);

        demo::master(false);

        activity::master(false);

        partition::master(false);

        iter_in_collections::master(false);

        extend::master(false);
    }
}
