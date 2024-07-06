pub fn master(show: bool) {
    if show {
        println!("Iterator with fold");

        let some_numbers = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

        println!(
            "{}",
            some_numbers
                .iter()
                .fold(0, |total_so_far, next_number| total_so_far + next_number)
        );
    }
}
