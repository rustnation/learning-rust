pub fn master(show: bool) {
    if show {
        println!("\n-- Immutable Borrowing of Variables");
        let one = "one".to_string();
        print(&one);
        println!("{}", one);
    }
}

fn print(value: &String) {
    println!("{}", value);
}
