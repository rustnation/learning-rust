pub fn master(show: bool) {
    if show {
        let _s1 = gives_ownership(); // gives_ownership moves its return value to s1

        let s2 = String::from("hallo"); // s2 comes into scope

        let s3 = takes_and_gives_back(s2); // s2 is move into takes_and_gives_back,
        // which also moves its return value into s3
        println!("{s3}");
    }
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s2 goes out of
  // and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move
    /*let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and moves out to the calling function*/
    String::from("yours")
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}
