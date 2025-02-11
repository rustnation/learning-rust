//! src/basics/control_flow.rs
pub fn master(show: bool) {
    if show {
        println!("\n--- Control Flow");

        if_expression();

        if_multiple_expressions();
    }
}

fn if_expression() {
    println!("--- If Expressions ---");

    let number = 7;

    if number < 77 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn if_multiple_expressions() {
    println!("--- If with Multiple Expressions ---");
    let number = 7;

    if number % 7 == 0 {
        println!("number is divisible by 7");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 7, 3, or 2");
    }
}
