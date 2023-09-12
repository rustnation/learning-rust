mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Move Vars");

        definition::master(true);
    }
}