//! src/numbers/try_into_method.rs
//! src/numbers/try_into_methods.rs

pub fn convert_try_into(show: bool) {
    if show {
        println!("--- Convert Try Into Method:");

        let a: i32 = 10;
        let b: u16 = 100;

        let b_ = b.into();

        if a < b_ {
            println!("Ten is less than one hundred.");
        }
    }
}
