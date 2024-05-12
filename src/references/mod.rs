mod definition;
mod ref_of_references;

pub fn master(show: bool) {
    if show {
        println!("\n-- References");

        definition::master(false);

        ref_of_references::master(false);
    }
}
