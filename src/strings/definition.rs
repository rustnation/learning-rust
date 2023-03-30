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

    // push a value into a String
    pushing_to_string();
}

fn pushing_to_string() {
    let mut s = String::from("Mutable string");

    println!("initial value of s String: {}", s);

    s.push_str(", this value was pushed to s variable");

    println!("After push: {}", s);
}
