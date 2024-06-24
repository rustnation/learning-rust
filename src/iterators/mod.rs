mod activity;
mod definition;
mod demo;
mod extend;
mod functional_style;
mod iter_in_collections;
mod nonfunctional_style;
mod partition;
mod scan_hash_table;

pub fn master(show: bool) {
    if show {
        println!("\n-- Iterators");

        definition::master(false);

        demo::master(false);

        activity::master(false);

        partition::master(false);

        iter_in_collections::master(false);

        extend::master(false);

        scan_hash_table::master(false);

        nonfunctional_style::master(false);

        functional_style::master(false);
    }
}
