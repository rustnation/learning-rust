pub fn master(show: bool) {
    if show {
        println!("-- Vectors");
        definition();
    }
}

fn definition() {
    let mut string_vector: Vec<&str> = vec!["one", "two", "three"];
    println!("Initial value of string_vector: {:?}", string_vector);
    // add a value to string_vector
    string_vector.push("four");
    println!("Final value of string_vector: {:?}", string_vector);
}
