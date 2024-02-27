//! src/numbers/comparing_numbers.rs

pub fn comparing_different_types(show: bool) {
    if show {
        println!("--- Comparing Different Types:");

        let a: i32 = 7;
        let b: u16 = 77;

        if a < (b as i32) {
            println!("Seven is less than seventy seven.");
        }
    }
}
