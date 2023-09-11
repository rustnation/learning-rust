pub fn master(show: bool) {
    if show {
        definition(false);

        build_vector_from_iterator(false);

        reverse_vector(true);
    }
}

fn definition(show: bool) {
    if show {
        println!("\n--- Vector Definition");

        let mut string_vector: Vec<&str> = vec!["one", "two", "three"];
        println!("Initial value of string_vector: {:?}", string_vector);

        // add a value to string_vector
        string_vector.push("four");
        println!("Append a value to string_vector: {:?}", string_vector);

        // vector sorting
        string_vector.sort();
        println!("string_vector sorted: {:?}", string_vector);

        // use a function to build a vector
        let v = build_vector();
        println!("Value of v: {:?}", v);
    }
}

// type inference
fn build_vector() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(7);
    v.push(77);
    v
}

// build a vector from the values produced by an iterator
fn build_vector_from_iterator(show: bool) {
    if show {
        println!("\n--- build a vector from the values produced by an iterator");
        let v: Vec<i32> = (0..7).collect();
        println!("v: {:?}", v);
    }
}

fn reverse_vector(show: bool) {
    if show {
        println!("\n--- reverse a vector");

        let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];
        println!("palindrome: {:?}", palindrome);
        palindrome.reverse();
        println!("palindrome reversed: {:?}", palindrome);

    }
}
