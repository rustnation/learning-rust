pub fn master() {
    println!("\n-- Immutable Borrowing of Variables");
    let one = "one".to_string();
    print(&one);
    println!("{}", one);
}

fn print(value: &String) {
    println!("{}", value);
}
