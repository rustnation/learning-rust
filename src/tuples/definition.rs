enum Access {
    Full,
}

fn one_two_three() -> (i32, i32, i32) {
    (1, 2, 3)
}

pub fn master(show: bool) {
    if show {
        println!("\n--- Definition");

        // Option 1
        let numbers = one_two_three();
        // Option 2
        let (x, y, z) = one_two_three();

        println!("{}, {}", x, numbers.0);
        println!("{}, {}", y, numbers.1);
        println!("{}, {}", z, numbers.2);

        // Combined with Enums
        let (employee, access) = ("Jake", Access::Full);
        println!("employee name: {}", employee);

        match access { Access::Full => println!("employee has full access") };
    }
}