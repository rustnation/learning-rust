mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Vectors");

        definition::master(true);
    }
}
