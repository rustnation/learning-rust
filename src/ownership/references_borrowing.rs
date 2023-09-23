pub fn master(show: bool) {
    if show {
        let s1 = String::from("hallo");

        let len = calculate_length(&s1);

        println!("The length of '{s1}' is {len}.");
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
