pub fn master() {
    println!("--- Moving a Value ---");
    let s = String::from("hallo");

    takes_ownership(s.clone()); // The compiler prevents us to pass the ownership

    println!("the value of s is: {s}");
}

fn takes_ownership(message: String) {
    println!("value received: {message}");
}
