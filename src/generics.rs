pub mod definition;

pub fn master(show: bool) {
    if show {
        println!("--- Generics Definition ---");
        definition::master();
    }
}
