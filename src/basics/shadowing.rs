//! src/basics/shadowing.rs
use crate::print_title;

pub fn master(show: bool) {
    if show {
        print_title("Shadowing");

        definition();

        transformation();
    }
}

fn definition() {
    print_title("Shadowing Definition");

    let x = 7;

    {
        let x = x * 7;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x in the outer scope is: {x}");
}

fn transformation() {
    print_title("Shadowing Transformation");

    let spaces = "       ";
    let spaces = spaces.len();

    println!("Now spaces is usize type: {}", spaces);
}
