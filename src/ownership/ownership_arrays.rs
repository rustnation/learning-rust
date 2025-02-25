pub fn master(show: bool) {
    if show {
        let registrations = [true, false, true];

        // this is valid because the first element of the array is store in stack and implements
        // copy trait
        let first = registrations[0];

        println!("first value: {first} and registrations: {registrations:?}");

        let languages = [String::from("Rust"), String::from("Go")];

        // cannot move out of type [String; 2]
        // let first = languages[0];
        let first = languages[0].clone();
        println!("first value: {first}, and languages: {languages:?}");
    }
}
