



pub fn conditional_loops_with_while() {
    println!("--- Conditional Loops with While ---");

    let mut number = 7;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

pub fn looping_through_collection_with_for() {
    println!("--- Looping Through a Collection with For ---");

    let a = [10, 20, 30, 40, 50 , 60, 77];
    let mut index = 0;

    while index < 7 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}

pub fn for_loop_elements() {
    println!("--- For Loop Elements ---");

    let a = [10, 20, 30, 40, 50, 60, 77];

    for element in a {
        println!("the value is: {element}");
    }
}

pub fn countdown_loop_with_rev() {
    println!("--- Countdown Loop with Rev ---");

    for number in (1..7).rev() {
        println!("the value is: {number}!");
    }

    println!("LIFTOFF!!!");
}