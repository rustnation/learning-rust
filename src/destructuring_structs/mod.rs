pub mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Destructuring Structs");

        definition::master(true);
    }
}
