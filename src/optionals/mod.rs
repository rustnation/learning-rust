mod activity;
mod definition;
mod unwrap_or;

pub fn master(show: bool) {
    if show {
        println!("\n-- Optionals");

        definition::master(false);

        activity::master(false);

        unwrap_or::master(false);
    }
}
