mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Results");

        definition::master(true);
    }
}