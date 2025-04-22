//! src/basics/functions.rs
pub fn index(show: bool) {
    if show {
        println!("\n--- Functions");

        print_labeled_measurement(7, 'h');

        function_with_parameters(7);
        function_multiple_parameters(7, 'h');

        let seven = function_with_return_value();
        println!("The value of seven is: {seven}");

        let result = sum(3, 4);
        display_result(result);
    }
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn function_with_parameters(x: i32) {
    println!("--- Function with Parameters ---");
    println!("The value of x is: {x}");
}

fn function_multiple_parameters(value: i32, unit_label: char) {
    println!("--- Function Multiple Parameters ---");
    println!("The values are: {value}{unit_label}");
}

fn function_with_return_value() -> i32 {
    println!("--- Function with Return Value ---");
    7
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn display_result(result: i32) {
    println!("\n-- Print the result of sum function");
    println!("result returned by sum function: {:?}", result);
}
