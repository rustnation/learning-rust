mod definition;

pub fn master(show: bool) {
    if show {
        println!("--- Concurrency");
        definition::master(false);
    }
}
