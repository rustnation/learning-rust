use crate::print_title;

pub fn master(show: bool) {
    if show {
        print_title("Arrays");

        demo(false);

        months(false);

        type_size(false);

        initial_values(false);
    }
}

fn demo(show: bool) {
    if show {
        print_title("Array Demo");

        let a = [1, 2, 3, 4, 5, 6, 7];
        println!("a values: {:?}", a);
    }
}

fn months(show: bool) {
    if show {
        print_title("Months Array");

        let months = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];

        println!("Months: {:?}", months);
    }
}

fn type_size(show: bool) {
    if show {
        print_title("Array Type & Size");

        let a: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
        println!("Values of a: {:?}", a);
    }
}

fn initial_values(show: bool) {
    if show {
        print_title("Array with initial values");

        let a = [7; 7];
        println!("Values of a: {:?}", a);
    }
}
