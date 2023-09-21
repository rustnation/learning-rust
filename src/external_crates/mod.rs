mod demo;
mod activity;

pub fn master(show: bool) {
    if show {
        println!("\n-- External Crates");

        demo::master(false);
        activity::master(true);
    }
}
