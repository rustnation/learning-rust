pub fn index(show: bool) {
    if show {
        let age = 77;

        {
            let is_handsome = true;
            println!("is handsome: {}", if is_handsome { "Yes" } else { "No" });
        } // is_handsome variable goes out of scope here

        println!("{}", age);

        // age variable exists here
    } // age variable goes out of scope here
}
