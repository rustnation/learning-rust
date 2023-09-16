mod definition;
mod activity;

pub fn master(show: bool) {
    if show {
        println!("\n-- Results");

        definition::master(false);

        activity::master(true);
    }
}