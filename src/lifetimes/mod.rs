mod definition;
mod example;

pub fn master(show: bool) {
    if show {
        println!("\n-- Lifetimes");

        definition::master(false);

        example::master(true);
    }
}
