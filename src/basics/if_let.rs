//! src/basics/if_let.rs
pub fn master(show: bool) {
    if show {
        println!("--- Using if in a let statement");

        let condition = true;

        let number = if condition { 5 } else { 6 }; // this is a bad design

        println!("The value of number is: {number}");
    }
}
