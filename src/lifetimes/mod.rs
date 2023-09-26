mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Lifetimes");

        definition::master(true);
    }
}
