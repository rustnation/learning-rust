mod activity;
mod definition;

mod furniture;

pub fn master(show: bool) {
    if show {
        // HashMap Definition
        definition::master(false);

        // HashMap Activity
        activity::master(false);

        // HashMap Furniture Activity
        furniture::master(false);
    }
}
