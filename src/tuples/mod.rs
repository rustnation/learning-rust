mod definition;
mod single_tuple;

pub fn master(show: bool) {
    if show {
        println!("\n-- Tuples");

        definition::master(false);

        single_tuple::master(false);
    }
}
