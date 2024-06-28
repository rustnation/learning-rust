mod activity;
mod definition;
mod demo;
mod extend;
mod functional_style;
mod impl_iterator;
mod iter_in_collections;
mod iterator_next;
mod mapping_and_filtering;
mod nonfunctional_style;
mod partition;
mod scan_hash_table;
mod without_for;
mod filter_map;

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

        without_for::master(false);

        iterator_next::master(false);

        impl_iterator::master(false);

        mapping_and_filtering::master(false);

        filter_map::master(false);
    }
}
