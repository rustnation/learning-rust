pub mod fah_to_cel;
pub mod fib;
pub mod christmas_carol;

pub fn master(show: bool) {
    if show {
        println!("\n--- Chapter 03");

        fah_to_cel::master();
        fib::master();
        christmas_carol::master();
    }
}

