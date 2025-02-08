pub fn master(show: bool) {
    if show {
        println!("--- Floating Points ---");
        let x = 2.0; // f64
        let y: f32 = 3.0; // f32
        println!("The value of x is: {x}");
        println!("The value of y is: {y}");
    }
}

// Type                 Precision
// f32                  6-9 digits of precision
// f64                  15-17 digits of precision
