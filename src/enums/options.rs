// The option type encodes the very common scenario in which a value could be something
// or it could be nothing.

pub fn master() {
    let some_number = Some(7);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    println!("The value of some_number is: {:?}", some_number);
    println!("The value of some_chart is: {:?}", some_char);
    println!("The value of absent_number is {:?}", absent_number);
}
