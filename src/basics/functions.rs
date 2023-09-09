pub fn function_with_parameters(x: i32) {
    println!("--- Function with Parameters ---");
    println!("The value of x is: {x}");
}

pub fn function_multiple_parameters(value: i32, unit_label: char) {
    println!("--- Function Multiple Parameters ---");
    println!("The values are: {value}{unit_label}");
}

pub fn function_with_return_value() -> i32 {
    println!("--- Function with Return Value ---");
    7
}

pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

pub fn display_result(result: i32) {
    println!("\n-- Print the result of sum function");
    println!("result returned by sum function: {:?}", result);
}
