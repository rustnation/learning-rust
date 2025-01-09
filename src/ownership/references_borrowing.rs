pub fn master(show: bool) {
    if show {
        let s1 = String::from("hallo");

        let len = calculate_length(&s1);

        println!("The length of '{s1}' is {len}.");
    }
}

// Has a reference to an object as a parameter instead of taking ownership of the value.
fn calculate_length(s: &str) -> usize {
    s.len()
}
