//! src/ownership/functions.rs
pub fn master(show: bool) {
    if show {
        println!("--- Moving a Value ---");
        let s = String::from("hallo");

        takes_ownership(s.clone()); // The compiler prevents us to pass the ownership

        println!("the value of s is: {s}");

        let x = 7;
        makes_copy(x);
    }
}

fn takes_ownership(message: String) {
    println!("value received: {message}");
}

fn makes_copy(some_integer: i32) {
    println!("some_integer value is: {some_integer}");
}
