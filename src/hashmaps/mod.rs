mod activity;
mod definition;

pub fn master(show: bool) {
    if show {
        // HashMap Definition
        definition::master(false);

        // HashMap Activity
        activity::master(true);
    }
}
