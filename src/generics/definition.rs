pub fn master() {
    println!("--- Generic Function Definition ---");
    using_generic_function();

    println!("--- Generic Struct Definition ---");
    using_struct_definition();

    println!("--- Generic Struct Multiple Values Definition ---");
    using_struct_multiple_values();
}

fn using_generic_function() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);

    println!("The largest number in number_list is: {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);

    println!("The largest number in number_list is: {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);

    println!("The largest char is: {result}");
}

// Generic definition
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if largest < item {
            largest = item;
        }
    }

    largest
}

// Struct Definition
struct Point<T> {
    x: T,
    y: T,
}

fn using_struct_definition() {
    let integer = Point { x: 7, y: 7 };
    let float = Point { x: 7.7, y: 7.7 };

    println!("The value of integer x is: {}", integer.x);
    println!("The value of integer y is: {}", integer.y);

    println!("The value of float x is: {}", float.x);
    println!("The value of float y is: {}", float.y);
}

// Multiple Values Definition
struct MPoint<T, U> {
    x: T,
    y: U,
}

fn using_struct_multiple_values() {
    let integers = MPoint { x: 7, y: 7 };
    let floats = MPoint { x: 7.7, y: 7.7 };
    let integer_and_float = MPoint { x: 7, y: 7.7 };

    println!("The value of integers x is: {}", integers.x);
    println!("The value of integers y is: {}", integers.y);

    println!("The value of floats x is: {}", floats.x);
    println!("The value of floats y is: {}", floats.y);

    println!("The value of integer_and_float x is: {}", integer_and_float.x);
    println!("The value of integer_and_float y is: {}", integer_and_float.y);
}
