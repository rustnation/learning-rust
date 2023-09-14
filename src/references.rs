mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- References");

        definition::master(true);
    }
}