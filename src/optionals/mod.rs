mod definition;
mod activity;

pub fn master(show: bool) {
    if show {
        println!("\n-- Optionals");

        definition::master(false);

        activity::master(false);
    }
}
