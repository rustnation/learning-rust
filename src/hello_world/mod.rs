//! src/hello_world/mod.rs
use crate::print_title;

pub fn index(show: bool) {
    if show {
        print_title("Hello World");
    }
}
