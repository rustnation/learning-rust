pub fn master(show: bool) {
    if show {
        println!("--- Slices ---");
        let slice_array: [i32; 100] = [0; 100];
        println!("slice: {:?}", &slice_array[5..8]);

        let s = String::from("hallo willkommen");
        let word = first_word(&s);
        println!("The value of word is: {}", word);
    }
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}