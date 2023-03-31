pub mod convert_fahrenheit_celsius;
pub mod fib;

pub fn master() {
    common::print_title("- Chapter 3");
    convert_fahrenheit_celsius::master();

    fib::master();
}

