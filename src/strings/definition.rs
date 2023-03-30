pub fn master() {
    let data = "This is a first literal string";

    // to_string method is available on any type that implements the Display trait
    let s = data.to_string();

    println!("{}", s);

    // the method also works on a literal directly
    let s1 = "This is a second literal string".to_string();

    println!("{}", s1);
}
