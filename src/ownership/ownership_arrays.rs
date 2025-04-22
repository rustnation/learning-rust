pub fn index(show: bool) {
    if show {
        let registrations = [true, false, true];

        // this is valid because the first element of the array is store in stack and implements
        // copy trait
        let first = registrations[0];

        println!("first value: {first} and registrations: {registrations:?}");

        let languages = [String::from("Rust"), String::from("Go")];

        // cannot move out of type [String; 2]
        // let first = languages[0];
        // this approach is valid but it is not idiomatic in Rust because we are duplicating a
        // value in the heap
        // let first = languages[0].clone();
        let first = &languages[0];
        println!("first value: {first}, and languages: {languages:?}");
    }
}
