pub fn master(show: bool) {
    if show {
        println!("-- Vectors");
        definition();

        let v = build_vector();
        println!("Value of v: {:?}", v);
    }
}

fn definition() {
    let mut string_vector: Vec<&str> = vec!["one", "two", "three"];
    println!("Initial value of string_vector: {:?}", string_vector);
    // add a value to string_vector
    string_vector.push("four");
    println!("Final value of string_vector: {:?}", string_vector);
}

// type inference
fn build_vector() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(7);
    v.push(77);
    v
}
