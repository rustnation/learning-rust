mod definition;
mod returning_values_from_loops;

pub fn master(show: bool) {
    if show {
        println!("\n-- Lifetimes");

        definition::master(false);

        returning_values_from_loops::master(false);
    }
}
