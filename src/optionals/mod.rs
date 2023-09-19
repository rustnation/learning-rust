mod definition;
mod demo;

pub fn master(show: bool) {
    if show {
        println!("\n-- Optionals");

        definition::master(false);

        demo::master(true);
    }
}
