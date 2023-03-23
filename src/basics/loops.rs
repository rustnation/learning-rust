pub fn returning_values_from_loops() {
    println!("--- Returning Values From Loops ---");

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 7;
        }
    };

    println!("The result is {result}");
}

pub fn loop_labels_multiple_loops() {
    println!("--- Loop Labels Multiple Loops ---");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

pub fn conditional_loops_with_while() {
    println!("--- Conditional Loops with While ---");

    let mut number = 7;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}