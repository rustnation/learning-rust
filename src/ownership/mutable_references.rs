pub fn master(show: bool) {
    if show {
        let mut s = String::from("hallo");

        change(&mut s);

        immutable_and_mutable_references();
    }
}

fn change(some_string: &mut String) {
    some_string.push_str(", Welt");
}

fn immutable_and_mutable_references() {
    let mut s = String::from("hallo");

    // r1 and r2 have an immutable reference to s
    // here there is a guarantee that no-one is going to change s
    let r1 = &s; // read-only
    let r2 = &s; // read-only
    println!("values of {r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // not problem
    println!("value of {r3}");
}
