pub fn master(show: bool) {
    if show {
        println!("--- Borrowing ---");

        let mut x;
        x = 42;
        println!("Initial x value: {x}");

        let y = &x;
        println!("Initial y value: {y}");

        x = 43;
        println!("Final x value: {x}");

        // conflicts with the assignment to x!
        //println!("Final y value: {y}");
    }
}