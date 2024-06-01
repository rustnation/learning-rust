pub fn master(show: bool) {
    if show {
        println!("--- Match in vector\n");

        let scores = vec![3, 4, 5, 6, 7];

        for index in 0..10 {
            match scores.get(index) {
                Some(number) => println!("The number is: {number}"),
                None => {}
            }
        }
    }
}
