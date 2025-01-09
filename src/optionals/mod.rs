mod activity;
mod definition;
mod unwrap_or_else;

pub fn master(show: bool) {
    if show {
        println!("\n-- Optionals");

        definition::master(false);

        activity::master(false);

        unwrap_or_else::master(false);
    }
}
