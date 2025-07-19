pub fn master(show: bool) {
    if show {
        println!("\n-- Module Basics");
    }
}

mod print_things {
    use std::fmt::Display;

    fn prints_one_thing<T: Display>(input: T) {
        println!("{input}");
    }
}
