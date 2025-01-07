mod definition;

pub fn master(show: bool) {
    if show {
        println!("-- Functions --");

        definition::master(false);
    }
}
