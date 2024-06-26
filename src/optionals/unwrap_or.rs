use std::i32;

pub fn master(show: bool) {
    if show {
        println!("None to wrap_or");

        let nothing: Option<i32> = None;

        println!("{}", nothing.unwrap_or(0));
    }
}
