mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Match Expression");

        definition::master(true);
    }
}
