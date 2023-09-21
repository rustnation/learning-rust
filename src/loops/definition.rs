pub fn master(show: bool) {
    if show {
        println!("-- Loops");

        returning_values_from_loops();

        loop_labels_multiple_loops();

        loop_print_numbers_1_to_4();
    }
}

pub fn returning_values_from_loops() {
    println!("\n--- Returning Values From Loops ---");

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
    println!("\n--- Loop Labels Multiple Loops ---");

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

pub fn loop_print_numbers_1_to_4() {
    println!("\n--- Loop to print numbers from 1 to 4");
    let mut i = 1;

    loop {
        println!("The value of i: {}", i);
        if i == 4 {
            break;
        }
        i += 1;
    }
}
