mod activity;
mod definition;
mod furniture;
mod hashmap_get;
mod hashmap_loop;

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
    }
}
