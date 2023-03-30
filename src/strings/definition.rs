pub fn master() {
    let data = "This is a first string literal";

    // to_string method is available on any type that implements the Display trait
    let s1 = data.to_string();

    println!("{}", s1);

    // the method also works on a literal directly
    let s2 = "This is a second string literal".to_string();

    println!("{}", s2);

    // we can also use the following function to create String from a string literal
    let s3 = String::from("This is a third string literal");

    println!("{}", s3);
}
