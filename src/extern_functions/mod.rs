mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Extern Functions");

        definition::master(true);
    }
}
