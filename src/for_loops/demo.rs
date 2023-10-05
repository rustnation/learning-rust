use crate::print_title;

pub fn definition(show: bool) {
    if show {
        print_title("For Loop Demo");

        let a = [10, 20, 30, 40, 50, 60, 70];

        for element in a {
            println!("The value is: {element}");
        }

        for_range();
    }
}

pub fn for_range() {
    for number in (1..8).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!!!");
}
