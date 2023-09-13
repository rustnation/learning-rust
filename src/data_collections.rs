mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Data Collections");

        definition::master(true);
    }
}