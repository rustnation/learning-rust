mod definition;
mod demo;
mod activity;

pub fn master(show: bool) {
    if show {
        println!("\n-- Optionals");

        definition::master(false);

        demo::master(false);

        activity::master(true);
    }
}
