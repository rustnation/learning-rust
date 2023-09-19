mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Mutable Reference");

        definition::master(true);
    }
}
