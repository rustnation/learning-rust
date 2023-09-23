pub fn master(show: bool) {
    if show {
        let mut s = String::from("hallo");

        change(&mut s);
    }
}

fn change(some_string: &mut String) {
    some_string.push_str(", Welt");
}
