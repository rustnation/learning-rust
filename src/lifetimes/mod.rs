mod definition;
mod example;
mod demo;

pub fn master(show: bool) {
    if show {
        println!("\n-- Lifetimes");

        definition::master(false);

        example::master(false);

        demo::master(false);
    }
}
