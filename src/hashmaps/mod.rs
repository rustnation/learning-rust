//! src/hashmaps/mod.rs
pub mod activity;
pub mod definition;
pub mod furniture;
pub mod hashmap_entry_method;
pub mod hashmap_get;
pub mod hashmap_loop;
pub mod hashmap_old_values;

pub fn master(show: bool) {
    if show {
        // HashMap Definition
        definition::master(false);

        // HashMap Activity
        activity::master(false);

        // HashMap Furniture Activity
        furniture::master(false);

        // HashMap in for
        hashmap_loop::master(false);

        // HashMap Get Method
        hashmap_get::master(false);

        // HashMap Old Values
        hashmap_old_values::master(false);

        // HashMap Entry Method
        hashmap_entry_method::master(false);
    }
}
