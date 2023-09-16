mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Optionals");

        definition::master(true);
    }
}