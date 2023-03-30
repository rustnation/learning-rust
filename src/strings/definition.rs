pub fn master() {
    let data = "This is a first string literal";

    // to_string method is available on any type that implements the Display trait
    let s = data.to_string();

    println!("{}", s);

    // the method also works on a literal directly
    let s1 = "This is a second string literal".to_string();

    println!("{}", s1);
}
