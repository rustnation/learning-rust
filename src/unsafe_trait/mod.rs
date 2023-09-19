mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Unsafe Trait");

        definition::master(true);
    }
}
