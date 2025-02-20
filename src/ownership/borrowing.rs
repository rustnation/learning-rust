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

// value is borrowing the value of the variable one defined on master function
// it is not taking ownership of the value of one stored in the heap
fn print(value: &String) {
    println!("{}", value);
} // the variables are going to be out of scope
// value is the owner of memory address that is pointing to the address of one in the heap
// one is the owner of the memory address on the heap

fn string_type() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{s}");

    // after this s is invalid
    let z = s;

    println!("z value: {z}");
}
