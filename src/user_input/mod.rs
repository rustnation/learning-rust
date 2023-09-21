mod demo;
mod activity;

pub fn master(show: bool) {
    if show {
        println!("\n--- User Input");

        demo::master(false);
        activity::master(true);
    }
}
