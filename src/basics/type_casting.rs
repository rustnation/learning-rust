pub fn index(show: bool) {
    if show {
        let integer_number: i32 = 7;
        let float_number = integer_number as f64;
        println!("float number: {}", float_number);

        // truncated integer
        let float_number = 7.77;
        let truncated_integer: i32 = float_number as i32;
        println!("truncated integer: {}", truncated_integer);
    }
}
