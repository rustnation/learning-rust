//! src/ownership/borrowing.rs
pub fn master(show: bool) {
    if show {
        println!("\n-- Immutable Borrowing of Variables");
        let one = "one".to_string();
        print(&one);
        println!("{}", one);

        string_type();
    }
}

fn print(value: &String) {
    println!("{}", value);
}

fn string_type() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{s}");

    // after this s is invalid
    let z = s;

    println!("z value: {z}");
}
